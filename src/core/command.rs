
// lib
    // std
use std::time;

    // serenity
use serenity::async_trait;
use serenity::builder::{
    CreateEmbed,
    CreateApplicationCommandOption,
    CreateActionRow,
    CreateButton,
    CreateComponents,
};

use serenity::model::interactions::application_command::{
    ApplicationCommandInteraction,
    ApplicationCommandInteractionDataOption,
};

use serenity::client::Context;


#[async_trait]
pub trait Command: Sync + Send {

    /// Returns the command's name
    fn name(self: &Self) -> String; 

    /// Returns the command's description
    fn description(self: &Self) -> String;

    /// Returns the command's options
    fn options(self: &Self) 
        -> Option<Vec<CreateApplicationCommandOption>> { None }

    // Runs the command
    async fn run(self: &Self, 
                 context: &Context,
                 command: &ApplicationCommandInteraction) {
        // init
        let channel = command.channel_id;
        let command_options = &command.data.options;
        
        // find ther user's id of the caller
        let author_id = command.user.id;
        
        // get command data
        let mut message_content = String::new();
        let mut has_content = false;
        if let Some(content) = self.content(context, command_options).await {
            message_content = content;
            has_content = true;
        }

        let mut message_embed = Vec::<CreateEmbed>::new();
        let mut has_embed = false;
        if let Some(embed) = self.embed(context, command_options).await {
            message_embed = vec![embed];
            has_embed = true;
        }

        let mut message_action_row = CreateComponents::default();
        let mut has_action_row = false;
        if let Some(action_row) = self.action_row(context, command_options).await {
            message_action_row.set_action_row(action_row);
            has_action_row = true;
        }

        let response = channel.send_message(
            &context.http,
            |message| {
                if has_content {
                    message.content(message_content);
                }
                if has_embed {
                    message.add_embeds(message_embed);
                }
                if has_action_row {
                    message.set_components(message_action_row);
                }

                message
            }
        ).await;

        if let Ok(message) = &response {
            let mut limit_duration = time::Duration::from_secs(60);
            if let Some(timeout) = self.timeout().await {
                limit_duration = timeout;
            }

            let message_id = message.id;

            // setup the interaction await
            let component_interaction = message
                .await_component_interaction(&context)
                .author_id(author_id)
                .message_id(message_id)
                .collect_limit(1)
                .timeout(limit_duration)
                .await;

            if let Some(interaction_response) = component_interaction {
                if interaction_response.data.custom_id == "sample_button_id" {
                    println!("blue");
                }
            } else {
                return;  // timout
            }
        }

        if let Err(error) = &response {
            panic!("Error in Command::run: Unable to send message ({}).", error);
        }
    }

    /// Returns the command's content
    async fn content(self: &Self, 
                     _context: &Context, 
                     _options: &Vec<ApplicationCommandInteractionDataOption>) 
        -> Option<String> { None }

    /// Returns the command's embed
    async fn embed(self: &Self, 
                   _context: &Context, 
                   _options: &Vec<ApplicationCommandInteractionDataOption>)
        -> Option<CreateEmbed> { None }

    /// Returns the command's action row
    async fn action_row(self: &Self, 
                        context: &Context, 
                        options: &Vec<ApplicationCommandInteractionDataOption>) 
        -> Option<CreateActionRow> { 
        
        // tells if the command's action row is empty
        let mut empty = true;
        let action_buttons = self.buttons(context, options).await;
        
        if let Some(_) = action_buttons {
            empty = false;
        }

        if empty {
            println!("The action_row is empty");
            return None;
        }

        let mut action_row = CreateActionRow::default();
        if let Some(buttons) = action_buttons {
            for button in buttons {
                action_row.add_button(button);
            }
        }

        Some(action_row)
    }
    
    /// Returns the command's buttons
    async fn buttons(self: &Self,
                     _context: &Context,
                     _options: &Vec<ApplicationCommandInteractionDataOption>)
        -> Option<Vec<CreateButton>> { None }

    /// Returns the timeout time
    async fn timeout(self: &Self) -> Option<time::Duration>;

}

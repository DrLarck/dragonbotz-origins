
// lib
    // std
use std::collections::HashMap;

    // serenity
use serenity::async_trait;
use serenity::prelude::EventHandler;
use serenity::client::Context;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::interactions::{
    Interaction,
    InteractionResponseType,
};

use serenity::model::interactions::application_command::{
    ApplicationCommand,
    ApplicationCommandInteraction,
};


// core
use crate::core::command::Command;


pub struct Bot {
    token: String,
    application_id: u64,
    test_guild_id: u64,
    commands: HashMap<String, Box<dyn Command>>,
}

impl Bot {

    /// Returns a new instance of Bot
    /// 
    /// ## Arguments:
    /// * token - the Bot's token
    /// * application_id - the application's id
    /// * test_guild_id - the test guild's id
    /// * commands - the Bot's commands
    pub fn new(token: String, application_id: u64, 
               test_guild_id: u64, commands: HashMap<String, Box<dyn Command>>)
            -> Self {
        Self {
            token,
            application_id,
            test_guild_id,
            commands,
        }
    }

    /// Returns the bot's token
    pub fn token(&self) -> String {
        self.token.clone()
    }

    /// Returns the bot's application id
    pub fn application_id(&self) -> u64 {
        self.application_id.clone()
    }

    /// Returns the bot's test guild id
    pub fn test_guild_id(&self) -> u64 {
        self.test_guild_id.clone()
    }

}

#[async_trait]
trait BotUtils {

    /// Returns the bot's commands
    async fn commands(self: &Self) -> &HashMap<String, Box<dyn Command>>;

    /// Executes the procedure for a slash command
    async fn execute_slash_command(self: &Self,
                                   context: &Context,
                                   command: &ApplicationCommandInteraction) {

        // get the name of the command that had been called
        let command_data = command.data.clone();
        let command_name = command_data.name.as_str();

        let bot_commands = self.commands().await;

        if !bot_commands.contains_key(command_name) {
            panic!("Error in Bot::execute_slash_command: Unable to find command \"{}\"", command_name)
        }
        
        // fins the command to run
        let command_to_run = &bot_commands[command_name];

        // get the command's option value
        let command_options = command_data
            .options;

        let command_content = command_to_run
            .content(&context, &command_options)
            .await;
        
        let command_embed = command_to_run
            .embed(&context, &command_options)
            .await;

        let command_action_row = command_to_run
            .action_row(&context, &command_options)
            .await;

        let interaction_creation = command.create_interaction_response(
            &context.http, 
            |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        let mut sendable = false;
                        
                        // check if the command has message content
                        if let Some(content) = command_content {
                            message.content(content);
                            sendable = true;
                        }

                        // check if the command has an embed
                        if let Some(embed) = command_embed {
                            message.add_embeds(vec![embed]);
                            sendable = true;
                        }

                        // check if the command has an action row
                        if let Some(action_row) = command_action_row {
                            message.components(|components| {
                                components.add_action_row(action_row)
                            });
                        }

                        // check if there is something to send
                        if !sendable {
                            panic!("Error in Bot::interaction_create: Nothing to be sent.");
                        }
                        
                        message
                    }
                )
            }
        ).await;

        if let Err(error) = interaction_creation {
            panic!("Error in Bot::interaction_create: Error creating response for command \"{}\": {}.", command_name, error)
        }

    }

}

#[async_trait]
impl BotUtils for Bot {

    async fn commands(&self) -> &HashMap<String, Box<dyn Command>> {
        &self.commands
    }

}

#[async_trait]
impl EventHandler for Bot {

    async fn ready(&self, context: Context, _ready: Ready) {

        // init
        let test_guild = GuildId(self.test_guild_id());
        let bot_commands = self.commands().await;

        // for every command stored in the bot, create a slash command
        let added_commands = test_guild.set_application_commands(
            &context.http, |command| {

                for (_key, cmd) in bot_commands {

                    command.create_application_command(|new_command| {
                        
                        // creating the interaction
                        new_command
                            .name(cmd.name())
                            .description(cmd.description());
                        
                        // adding options to the interaction
                        if let Some(options) = cmd.options() {
                            new_command.set_options(options);
                        }

                        println!("Created command: \"{}\"", cmd.name());

                        new_command
                    });
                }

                command
                
            }).await;
        
        if let Err(error) = added_commands {
            panic!("Error in Bot::ready: Error while setting up test guild slash commands: {}", error)
        }
    }

    async fn interaction_create(&self, context: Context, 
                                interaction: Interaction) {

        // if the interaction is a slash command
        if let Some(command) = &interaction.clone().application_command() {
            self.execute_slash_command(&context, &command).await;
        }

        // if the interaction is component interaction (button, etc.)
        if let Some(component) = interaction.clone().message_component() {
            println!("A button had been clicked.");
        }

    }

}

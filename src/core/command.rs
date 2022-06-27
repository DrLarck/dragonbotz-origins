
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
};

use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOption;
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

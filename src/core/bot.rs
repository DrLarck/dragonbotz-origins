
// lib
    // std
use std::collections::HashMap;

    // serenity
use serenity::async_trait;
use serenity::prelude::EventHandler;
use serenity::client::Context;
use serenity::model::{
    gateway::Ready,
    id::GuildId,
    interactions::application_command::ApplicationCommand,
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

    /// Returns the bot's commands
    pub fn commands(&self) -> &HashMap<String, Box<dyn Command>> {
        &self.commands
    }

}

#[async_trait]
impl EventHandler for Bot {

    async fn ready(&self, context: Context, _ready: Ready) {

        // init
        let test_guild = GuildId(self.test_guild_id());
        let bot_commands = self.commands();
        let mut added_commands = Vec::<ApplicationCommand>::new();

        // for every command stored in the bot, create a slash command
        let temp_added_commands = test_guild.set_application_commands(
            &context.http, |command| {

                for (_key, cmd) in bot_commands {
                    command.create_application_command(|new_command| {
                        new_command
                            .name(cmd.name())
                            .description(cmd.description())
                    });
                }

                command
                
            }).await;
        
        if let Ok(temp) = temp_added_commands {
            added_commands = temp;
        } else if let Err(error) = temp_added_commands {
            panic!("Error in Bot::ready: Error while setting up test guild slash commands: {}", error)
        }

        // becomes immutable
        let added_commands = added_commands;

        // displays all the added commands
        for command in added_commands {
            println!("Command {:?} had been added", command);
        }

    }

}

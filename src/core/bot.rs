
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
    interactions::{
        Interaction,
        InteractionResponseType,
        application_command::ApplicationCommand,
    }
};
use serenity::builder::CreateApplicationCommandOption;


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
                        
                        // creating the interaction
                        new_command
                            .name(cmd.name())
                            .description(cmd.description());
                        
                        // adding options to the interaction
                        if let Some(options) = cmd.options() {
                            new_command.set_options(options);
                        }

                        new_command
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

    async fn interaction_create(&self, context: Context, 
                                interaction: Interaction) {

        // the command that had been used
        let called = match interaction.application_command() {
            Some(command) => command,
            None => panic!("Error in Bot::interaction_create: Unable to retrieve the command that had been called."),
        };

        // get the name of the command that had been called
        let command_data = called.data.clone();
        let command_name = command_data.name.as_str();

        let bot_commands = self.commands();
        if bot_commands.contains_key(command_name) {
            
            let command_to_run = &bot_commands[command_name];
            let command_content = command_to_run
                .content(&context)
                .await
                .clone();
            
            let command_embed = command_to_run
                .embed(&context)
                .await
                .clone();

            let interaction_creation = called.create_interaction_response(&context.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        let mut sendable = false;

                        if let Some(content) = command_content {
                            message.content(content);
                            sendable = true;
                        }

                        if let Some(embed) = command_embed {
                            message.add_embed(embed);
                            sendable = true;
                        }

                        if !sendable {
                            panic!("Error in Bot::interaction_create: Nothing to be sent.");
                        }

                        message
                    })
            }).await;

            if let Err(error) = interaction_creation {
                panic!("Error in Bot::interaction_create: Error creating response for command \"{}\": {}.", command_name, error)
            }

        }

    }

}


// lib
    // std
use std::collections::HashMap;

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

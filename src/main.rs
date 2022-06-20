
// mods
mod commands;
mod core;
mod utils;

// lib
    // std
use std::collections::HashMap;

    // serenity
use serenity::client::{
    Client,
};

use serenity::prelude::*;

// commands
use crate::commands::ping::PingCommand;

// core
use crate::core::bot::Bot;
use crate::core::command::Command;

// utils
use crate::utils::config::Config;


#[tokio::main]
async fn main() {

    let mut commands = HashMap::<String, Box<dyn Command>>::new();

    // inserts commands into commands hashmap
    let commands_array: [Box<dyn Command>; 1] = [Box::new(PingCommand)];
    for command in commands_array {
        commands.insert(command.name(), command);
    }
    
    let bot = Bot::new(
        Config::environment_var::<String>("DRAGONBOTZ_TOKEN"),
        Config::environment_var::<u64>("DRAGONBOTZ_APP_ID"),
        Config::environment_var::<u64>("DRAGONBOTZ_TEST_GUILD_ID"),
        commands,
    );

    let application_id = bot.application_id();
    let pre_client = Client::builder(bot.token(), GatewayIntents::default())
        .event_handler(bot)
        .application_id(application_id)
        .await;
    
    if let Ok(mut client) = pre_client {
        
        match client.start().await {
            Ok(()) => (),
            Err(error) => panic!("Error in main: Unable to start the client: {}", error)
        }

    } else if let Err(error) = pre_client {
        panic!("Error in main: Error creating client: {}", error)
    }

}

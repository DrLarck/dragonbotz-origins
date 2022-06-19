
// mods
mod core;
mod utils;

// lib
    // serenity
use serenity::client::{
    Client,
};

use serenity::prelude::*;

// core
use crate::core::bot::Bot;

// utils
use crate::utils::config::Config;


#[tokio::main]
async fn main() {
    
    let bot = Bot::new(
        Config::environment_var::<String>("DRAGONBOTZ_TOKEN"),
        Config::environment_var::<u64>("DRAGONBOTZ_APP_ID"),
        Config::environment_var::<u64>("DRAGONBOTZ_TEST_GUILD_ID"),
    );

    let pre_client = Client::builder(bot.token(), GatewayIntents::empty())
        .application_id(bot.application_id())
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

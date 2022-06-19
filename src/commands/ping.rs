
// lib
    // serenity
use serenity::async_trait;
use serenity::builder::CreateEmbed;

// core
use crate::core::command::Command;

// utils
use crate::utils::embed::Embed;
use crate::utils::other::Other;


pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {

    fn name(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "A simple ping command.".to_string()
    }

    async fn content(&self) -> Option<String> {
        Some("Pong ! Eheh".to_string())
    }

    async fn embed(&self) -> Option<CreateEmbed> {
        let mut embed = Embed::new();

        embed.title("This is title".to_string());
        embed.description("This is a short description".to_string());
        embed.footer(Other::bot_avatar_url(), "test".to_string());
        embed.add_field("This is a new field".to_string(), "this is the value of my field".to_string(), false);
        embed.add_field("This is a field under my top field".to_string(), "value lol".to_string(), true);
        embed.add_field("This is a field on the right".to_string(), "another value :)".to_string(), true);

        Some(embed.create())
    }

}

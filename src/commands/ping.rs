
// lib
    // serenity
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::client::Context;

// core
use crate::core::command::Command;

// utils
use crate::utils::embed::Embed;


pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {

    fn name(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "A simple ping command.".to_string()
    }

    async fn content(&self, _: &Context) -> Option<String> {
        Some("Pong ! Eheh".to_string())
    }

    async fn embed(&self, context: &Context) -> Option<CreateEmbed> {
        let mut embed = Embed::new(context);

        embed.title("This is title".to_string());
        embed.description("This is a short description".to_string());
        embed.image("https://i.imgur.com/jgseMqC.png".to_string());
        embed.thumbnail("https://i.imgur.com/JwEHf59.png".to_string());
        embed.add_field("This is a new field".to_string(), "this is the value of my field".to_string(), false);
        embed.add_field("This is a field under my top field".to_string(), "value lol".to_string(), true);
        embed.add_field("This is a field on the right".to_string(), "another value :)".to_string(), true);

        Some(embed.create())
    }

}

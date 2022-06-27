
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
use serenity::client::Context;
use serenity::model::interactions::application_command::{
    ApplicationCommandOptionType,
    ApplicationCommandInteractionDataOption,
    ApplicationCommandInteractionDataOptionValue,
};
use serenity::model::interactions::message_component::ButtonStyle;

// core
use crate::core::command::Command;

// utils
use crate::utils::embed::Embed;
use crate::utils::other::Utils;


pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {

    fn name(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "A simple ping command.".to_string()
    }

    fn options(self: &Self) -> Option<Vec<CreateApplicationCommandOption>> {
        let mut options = Vec::<CreateApplicationCommandOption>::new();

        options.push(
            CreateApplicationCommandOption::default()
                .kind(ApplicationCommandOptionType::String)
                .name("name")
                .description("Your name")
                .clone()
        );

        Some(options)
    }

    async fn content(&self, 
                     _: &Context, 
                     options: &Vec<ApplicationCommandInteractionDataOption>) 
        -> Option<String> {
        
        let mut user_name = String::new();
        
        // get name option
        if let Some(name_option) = options.get(0) {

            let name = Utils::option_value(name_option);
            if let ApplicationCommandInteractionDataOptionValue::String(value) = name {
                user_name = value;
            } else {
                return Some("error".to_string());
            }

        }

        Some(format!("Pong ! Eheh: {}", user_name).to_string())
    }

    async fn embed(&self, 
                   context: &Context,
                    _: &Vec<ApplicationCommandInteractionDataOption>) 
        -> Option<CreateEmbed> {

        let mut embed = Embed::new(context);

        let stats = "♥️ Health: **666**/**666**
        🛡️ Defense: **890**";

        embed.title("This is title".to_string());
        embed.description(stats.to_string());
        embed.image("https://i.imgur.com/jgseMqC.png".to_string());
        embed.thumbnail("https://i.imgur.com/JwEHf59.png".to_string());
        embed.add_field("This is a new field".to_string(), "this is the value of my field".to_string(), false);
        embed.add_field("This is a field under my top field".to_string(), "value lol".to_string(), true);
        embed.add_field("This is a field on the right".to_string(), "another value :)".to_string(), true);

        Some(embed.create())
    }

    async fn buttons(&self,
                     _context: &Context,
                     _options: &Vec<ApplicationCommandInteractionDataOption>)
        -> Option<Vec<CreateButton>> {
        
        let mut buttons = Vec::<CreateButton>::new();

        let mut button = CreateButton::default();
        button.custom_id("sample_button_id");
        button.label("This is a button");

        let mut sec_button = CreateButton::default();
        sec_button.custom_id("danger_button");
        sec_button.label("Danger");
        sec_button.style(ButtonStyle::Danger);

        let mut link = CreateButton::default();
        link.label("link");
        link.url("https://google.com");
        link.style(ButtonStyle::Link);

        let mut sec = CreateButton::default();
        sec.custom_id("sec");
        sec.label("sec");
        sec.style(ButtonStyle::Secondary);

        buttons.push(button);
        buttons.push(sec_button);
        buttons.push(link);
        buttons.push(sec);
        Some(buttons)
    }

    async fn timeout(&self) -> Option<time::Duration> {
        Some(time::Duration::from_secs(60))
    }

}

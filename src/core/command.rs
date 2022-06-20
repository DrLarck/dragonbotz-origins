
// lib
    // serenity
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::client::Context;


#[async_trait]
pub trait Command: Sync + Send {

    /// Returns the command's name
    fn name(self: &Self) -> String; 

    /// Returns the command's description
    fn description(self: &Self) -> String;

    /// Returns the command's content
    async fn content(self: &Self, context: &Context) -> Option<String> { None }

    /// Returns the command's embed
    async fn embed(self: &Self, context: &Context) -> Option<CreateEmbed> { None }

}


// lib
    // serenity
use serenity::client::Context;


pub struct Other;

impl Other {

    /// Returns the bot's avatar url
    pub fn bot_avatar_url(context: &Context) -> String {
        let cache = context.clone().cache;
        let bot = cache.current_user();

        bot.face()
    }

    /// Returns the bot's name
    pub fn bot_name(context: &Context) -> String {
        let cache = context.clone().cache;
        let bot = cache.current_user();

        bot.name
    }

    /// Returns the bot's color
    pub fn bot_color() -> (u8, u8, u8) {
        // Goku's orange
        (232, 97, 45)
    }

}

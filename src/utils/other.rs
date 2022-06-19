
// lib
    // serenity
use serenity::model::user::CurrentUser;


pub struct Other;

impl Other {

    /// Returns the bot's avatar url
    pub fn bot_avatar_url() -> String {
        let bot = CurrentUser::default();

        bot.face()
    }

    /// Returns the bot's name
    pub fn bot_name() -> String {
        let bot = CurrentUser::default();

        bot.name
    }

    /// Returns the bot's color
    pub fn bot_color() -> (u8, u8, u8) {
        // Goku's orange
        (232, 97, 45)
    }

}

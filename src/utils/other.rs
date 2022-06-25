
// lib
    // serenity
use serenity::client::Context;
use serenity::model::interactions::application_command::{
    ApplicationCommandInteractionDataOption,
    ApplicationCommandInteractionDataOptionValue,
};


pub struct Utils;

impl Utils {

    /// Returns the bot's avatar url
    /// 
    /// ## Arguments:
    /// * context - the application context
    pub fn bot_avatar_url(context: &Context) -> String {
        let cache = context.clone().cache;
        let bot = cache.current_user();

        bot.face()
    }

    /// Returns the bot's name
    /// 
    /// ## Arguments:
    /// * context - the application context
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

    /// Returns the value of the option
    ///
    /// ## Arguments:
    /// * option - the option to get resolved value from
    pub fn option_resolved(option: &ApplicationCommandInteractionDataOption)
        -> Result<ApplicationCommandInteractionDataOptionValue, String> {
        
        if let Some(resolved) = option.clone().resolved {
            return Ok(resolved);
        } else {
            return Err("Option resolved value not found".to_string());
        }

    }

    /// Returns the option's value
    /// 
    /// ## Arguments:
    /// * option - the option to get value from
    pub fn option_value(option: &ApplicationCommandInteractionDataOption)
        -> ApplicationCommandInteractionDataOptionValue {
        
        match Utils::option_resolved(option) {
            Ok(value) => value,
            Err(error) => panic!("{}", error),
        }

    }

}

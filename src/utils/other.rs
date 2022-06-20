
// lib
    // serenity
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::interactions::application_command::{
    ApplicationCommandOption,
    ApplicationCommandOptionType,
};
use serenity::builder::CreateApplicationCommandOption;


pub struct Other;

impl Other {

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

    /// Converts Application Command Option into Create Application 
    /// Command Option
    /// 
    /// ## Arguments:
    /// * option - the option to convert
    fn convert_option_to_builder(option: &ApplicationCommandOption)
        -> CreateApplicationCommandOption {
        
        let mut builder = CreateApplicationCommandOption::default();

        // set builder
        builder.name(&option.name);
        builder.kind(option.kind);
        builder.description(&option.description);
        builder.required(option.required);
        
        // adds the option's choices to the command
        for choice in &option.choices {
            if let ApplicationCommandOptionType::Integer = option.kind {

                let mut _value = 0;
                if let Some(integer) = choice.value.as_i64() {
                    _value = integer as i32;
                } else {
                    panic!("Error in Other::convert_option_to_builder: Unable to convert choice \"{}\" option to i32.", choice.name)
                }

                builder.add_int_choice(&choice.name, _value);
                continue;

            } else if let ApplicationCommandOptionType::String = option.kind {

                let mut _value = String::new();
                if let Some(string) = choice.value.as_str() {
                    _value = string.to_string();
                } else {
                    panic!("Error in Other::convert_option_to_builder: Unable to convert choice \"{}\" option to String.", choice.name)
                }

                builder.add_string_choice(&choice.name, _value);
                continue;

            } else if let ApplicationCommandOptionType::Number = option.kind {

                let mut _value: f64 = 0.0;
                if let Some(number) = choice.value.as_f64() {
                    _value = number;
                } else {
                    panic!("Error in Other::convert_option_to_builder: Unable to convert choice \"{}\" option to f32.", choice.name)
                }

                builder.add_number_choice(&choice.name, _value);
                continue;

            } else {
                panic!("Error in Other::convert_option_to_builder: Unknown type for \"{}\" option.", choice.name)
            }
        }

        builder.channel_types(&option.channel_types);
    
        if let Some(value) = option.clone().min_value {
            builder.min_int_value(value.clone());
            
            if let Some(converted) = value.as_f64() {
                builder.min_number_value(converted);
            } else {
                panic!("Error in Other::convert_option_to_builder: Unable to convert min number value of \"{}\" to f64.", option.name)
            }
        }

        if let Some(value) = option.clone().max_value {
            builder.max_int_value(value.clone());

            if let Some(converted) = value.as_f64() {
                builder.max_number_value(converted);
            } else {
                panic!("Error in Other::convert_option_to_builder: Unable to convert max number value of \"{}\" to f64.", option.name)
            }
        }

        builder
    }

}

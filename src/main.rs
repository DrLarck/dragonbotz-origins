
// mods
mod core;
mod utils;

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

    

}

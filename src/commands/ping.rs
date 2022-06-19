
// lib
    // serenity
use serenity::async_trait;

// core
use crate::core::command::Command;


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

}

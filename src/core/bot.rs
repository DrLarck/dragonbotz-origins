
pub struct Bot {
    token: String,
    application_id: u64,
    test_guild_id: u64,
}

impl Bot {

    /// Returns a new instance of Bot
    /// 
    /// ## Arguments:
    /// * token - the Bot's token
    /// * application_id - the application's id
    /// * test_guild_id - the test guild's id
    pub fn new(token: String, application_id: u64, test_guild_id: u64) -> Self {
        Self {
            token,
            application_id,
            test_guild_id,
        }
    }

    /// Returns the bot's token
    pub fn token(&self) -> String {
        self.token.clone()
    }

    /// Returns the bot's application id
    pub fn application_id(&self) -> u64 {
        self.application_id.clone()
    }

}

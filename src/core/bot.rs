
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

}

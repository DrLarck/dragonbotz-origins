
use std::env::{
    var,
    VarError,
};

pub struct Config;

impl Config {

    /// Converts and returns the environment variable value
    /// 
    /// ## Arguments
    /// * name - the name (key) of the environment variable
    /// 
    /// ## Examples
    /// ```rust
    /// let token = Config::environment_var::<u64>("key");  // returns the token converted into u64
    /// ```
    pub fn environment_var<T>(name: &str) -> T
        where T: std::str::FromStr {

        // tries to find the environment variable
        match var(name) {
            Ok(value) => {

                // tries to convert the environment variable
                match value.parse::<T>() {
                    Ok(converted) => converted,
                    Err(_) => panic!("Error in Config: Unable to parse \"{}\" to desired type.", name),
                }

            }
            Err(VarError::NotPresent) => panic!("Error in Config: Unable to find environment variable \"{}\".", name),
            Err(_) => panic!("Error in Config: The environment variable \"{}\" contains invalid characters.", name),
        }

    }

}

#[cfg(test)]
mod config_test {

    use super::*;

    #[test]
    #[should_panic]
    fn should_panic_if_invalid_env_var() {
        Config::environment_var::<String>("KEY_THAT_DOESNT_EXIST");
    }

    #[test]
    fn should_return_valid_token() {
        let env = var("DRAGONBOTZ_TOKEN");
        let mut token = String::new();

        if let Ok(t) = env {
            token = t;
        }

        assert_eq!(token, Config::environment_var::<String>("DRAGONBOTZ_TOKEN"));
    }

    #[test]
    #[should_panic]
    fn should_panic_when_bad_conversion() {
        Config::environment_var::<u64>("DRAGONBOTZ_TOKEN");
    }

}

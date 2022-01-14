use serde::{Deserialize, Serialize};
use toml;

use crate::error::EnvmError;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    local: String,
    pattern: String,
    template: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            local: String::from(".env"),
            pattern: String::from(".env.{}"),
            template: String::from(".env.example"),
        }
    }

    pub fn from(contents: &str) -> Result<Config, EnvmError> {
        let config: Config =
            toml::from_str(contents).map_err(|_| EnvmError::FailedToParseConfig)?;
        Ok(config)
    }

    pub fn local(&self) -> &String {
        &self.local
    }

    pub fn pattern(&self) -> &String {
        &self.pattern
    }

    pub fn template(&self) -> &String {
        &self.template
    }

    pub fn to_string(&self) -> String {
        toml::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prase_config() -> Result<(), EnvmError> {
        let config = Config::from(
            r#"
            local = ".env"
            pattern = ".env.{}"
            example = ".env.example"
        "#,
        )?;
        assert_eq!(config.local, ".env");
        assert_eq!(config.pattern, ".env.{}");
        assert_eq!(config.template, ".env.example");
        Ok(())
    }
}

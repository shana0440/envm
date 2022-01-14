use serde::Deserialize;
use toml;

use crate::error::EnvmError;

#[derive(Deserialize, Debug)]
pub struct Config {
    local: String,
    pattern: String,
}

impl Config {
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
        "#,
        )?;
        assert_eq!(config.local, ".env");
        assert_eq!(config.pattern, ".env.{}");
        Ok(())
    }
}

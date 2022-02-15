use confy;
use serde::{Deserialize, Serialize};

use crate::error::EnvmError;

#[derive(Deserialize, Serialize, Debug, Default)]
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

    pub fn load(path: &str) -> Result<Config, EnvmError> {
        let config: Config = confy::load_path(path).map_err(|_| EnvmError::FailedToParseConfig)?;
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

    pub fn store(&self, path: &str) {
        confy::store_path(path, self).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use tempfile::NamedTempFile;

    #[test]
    fn prase_config() -> Result<(), EnvmError> {
        let mut file = NamedTempFile::new().unwrap();
        let content = r#"
            local = ".env"
            pattern = ".env.{}"
            template = ".env.example"
        "#;
        file.write_all(content.as_bytes()).unwrap();
        let config = Config::load(&file.path().to_str().unwrap())?;
        assert_eq!(config.local, ".env");
        assert_eq!(config.pattern, ".env.{}");
        assert_eq!(config.template, ".env.example");
        Ok(())
    }
}

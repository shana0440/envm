use std::{env, fs, path::{Path, PathBuf}};

use crate::config::Config;
use crate::error::EnvmError;

pub struct Repository {
    worktree: PathBuf,
    config: Config,
}

impl Repository {
    pub fn new() -> Result<Repository, EnvmError> {
        let worktree = env::current_dir().unwrap().join(".envm");
        let config_path = worktree.join("config");
        let contents = fs::read_to_string(config_path)
            .map_err(|_| EnvmError::MissingConfigFile)?;
        let config = Config::from(&contents)?;
        Ok(Repository {
            worktree,
            config,
        })
    }

    pub fn worktree(&self) -> &Path {
        self.worktree.as_path()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

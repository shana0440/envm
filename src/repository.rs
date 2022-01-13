use std::{env, fs, path::{Path, PathBuf}};

pub mod config;
pub mod environment;

use crate::repository::config::Config;
use crate::error::EnvmError;

pub struct Repository {
    path: PathBuf,
    envm_path: PathBuf,
    config: Config,
}

impl Repository {
    pub fn new() -> Result<Repository, EnvmError> {
        let path = match lookup_repository(env::current_dir().unwrap()) {
            Some(path) => path,
            None => return Err(EnvmError::NotEnvmRepository),
        };
        let envm_path = path.join(".envm");
        let config_path = envm_path.join("config");
        let contents = fs::read_to_string(config_path)
            .map_err(|_| EnvmError::MissingConfigFile)?;
        let config = Config::from(&contents)?;
        Ok(Repository {
            path,
            envm_path,
            config,
        })
    }

    pub fn envm_path(&self) -> &Path {
        self.envm_path.as_path()
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

fn lookup_repository(dir: PathBuf) -> Option<PathBuf> {
    let mut path = dir.as_path();
    while !path.join(".envm").exists() {
        match path.parent() {
            Some(p) => path = p,
            None => return None,
        }
    }
    return Some(path.to_path_buf());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_a_repository() {
        let root = Path::new("/not/a/repository").to_path_buf();
        assert_eq!(lookup_repository(root), None);
    }
}

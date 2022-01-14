use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub mod config;
pub mod environment;
pub mod path;

use crate::error::EnvmError;
use crate::repository::config::Config;

pub struct Repository {
    path: PathBuf,
    config: Config,
}

impl Repository {
    pub fn new() -> Result<Repository, EnvmError> {
        let path = match lookup_repository(env::current_dir().unwrap()) {
            Some(path) => path,
            None => return Err(EnvmError::NotEnvmRepository),
        };
        let config_path = path::get_config_path(&path);
        let contents = fs::read_to_string(config_path).map_err(|_| EnvmError::MissingConfigFile)?;
        let config = Config::from(&contents)?;
        Ok(Repository { path, config })
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

use std::{env, fs, path::{Path, PathBuf}};

use crate::config::Config;
use crate::error::EnvmError;

pub struct Repository {
    worktree: PathBuf,
    config: Config,
}

impl Repository {
    pub fn new() -> Result<Repository, EnvmError> {
        let worktree = match lookup_repository(env::current_dir().unwrap()) {
            Some(path) => path.join(".envm"),
            None => return Err(EnvmError::NotEnvmRepository),
        };
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

use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub mod config;
pub mod environment;
pub mod path;

use crate::error::EnvmError;
use crate::repository::config::Config;
use crate::repository::environment::EnvType;

pub struct Repository {
    path: PathBuf,
    config: Config,
    current_env: EnvType,
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
        let head_path = path::get_head_path(&path);
        let contents = fs::read_to_string(head_path).map_err(|_| EnvmError::MissngHeadFile)?;
        let current_env = EnvType::from(&contents);
        Ok(Repository { path, config, current_env })
    }

    fn get_environment_filename(&self, env: &str) -> PathBuf {
        let pattern = self.config.pattern();
        path::get_env_path(&self.path, pattern, env)
    }

    // Use to get the main environment file path
    fn get_local_environment_filename(&self) -> PathBuf {
        let local_env_name = self.config.local();
        self.path.join(local_env_name)
    }

    fn set_head(&self, env: &str) {
        let env = EnvType::from(env);
        let data = format!("{}", env);
        let head_path = path::get_head_path(&self.path);
        fs::write(head_path, data).unwrap();
    }

    pub fn use_environment(&self, env: &str) -> Result<(), EnvmError> {
        let local_env = self.get_local_environment_filename();
        let backup_path = path::get_local_backup_path(&self.path);
        if matches!(self.current_env, EnvType::Local) {
            fs::copy(&local_env, &backup_path)
                .map_err(|_| EnvmError::FailedToBackupLocalEnvironment)?;
        }

        let copy = |target, err| {
            if Path::new(target).exists() {
                fs::copy(target, local_env).unwrap();
                Ok(())
            } else {
                Err(err)
            }
        };
        match EnvType::from(env) {
            EnvType::Local => {
                copy(&backup_path, EnvmError::MissingBackupEnvironment)?;
            }
            EnvType::Other(_) => {
                let target_env = self.get_environment_filename(env);
                copy(
                    &target_env,
                    EnvmError::MissingTargetEnvironment(String::from(env)),
                )?;
            }
        }
        self.set_head(env);
        Ok(())
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

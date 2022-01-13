use std::{fs, fmt, path::{PathBuf, Path}};

use crate::error::EnvmError;
use crate::repository::Repository;
use crate::repository::config::Config;

// To identify the current environment is local or other, since in most case we won't have another
// file for local environment, so we need to backup the environment file if we are using local
// environment configuration, then we can switch back to local environment later.
#[derive(Debug)]
enum EnvType {
    Local,
    Other(String),
}

impl EnvType {
    fn from(contents: &str) -> EnvType {
        let contents = contents.trim();
        if contents == "local" {
            EnvType::Local
        } else {
            EnvType::Other(String::from(contents))
        }
    }
}

impl fmt::Display for EnvType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            EnvType::Local => write!(f, "local"),
            EnvType::Other(value) => write!(f, "{}", value),
        }
    }
}

pub struct Environment<'a> {
    config: &'a Config,
    repo_path: &'a Path,
    head_path: PathBuf,
    backup_path: PathBuf,
    current_env: EnvType,
}

impl<'a> Environment<'a> {
    pub fn new(repo: &Repository) -> Result<Environment, EnvmError> {
        let head_path = repo.envm_path().join("HEAD");
        let contents = fs::read_to_string(&head_path).map_err(|_| EnvmError::MissingConfigFile)?;
        let current_env = EnvType::from(&contents);
        let backup_path = repo.envm_path().join(".env.backup");
        Ok(Environment {
            config: repo.config(),
            repo_path: repo.path(),
            head_path,
            backup_path,
            current_env,
        })
    }

    fn get_environment_filename(&self, env: &str) -> PathBuf {
        let pattern = self.config.pattern();
        let filename = pattern.replace("{}", env);
        self.repo_path.join(filename)
    }

    // Use to get the main environment file path
    fn get_local_environment_filename(&self) -> PathBuf {
        let local_env_name = self.config.local();
        self.repo_path.join(local_env_name)
    }

    fn set_head(&self, env: &str) {
        let env = EnvType::from(env);
        let data = format!("{}", env);
        fs::write(&self.head_path, data).unwrap();
    }

    pub fn use_environment(&self, env: &str) -> Result<(), EnvmError> {
        let local_env = self.get_local_environment_filename();
        if matches!(self.current_env, EnvType::Local) {
            fs::copy(&local_env, &self.backup_path).map_err(|_| EnvmError::FailedToBackupLocalEnvironment)?;
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
                copy(&self.backup_path, EnvmError::MissingBackupEnvironment)?;
            }
            EnvType::Other(_) => {
                let target_env = self.get_environment_filename(env);
                copy(&target_env, EnvmError::MissingTargetEnvironment(String::from(env)))?;
            }
        }
        self.set_head(env);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_local_head() {
        let env = EnvType::from("local");
        assert!(matches!(env, EnvType::Local));
    }

    #[test]
    fn get_other_head() {
        let env = EnvType::from("dev");
        assert!(matches!(env, EnvType::Other(v) if v == "dev"));
    }

    #[test]
    fn get_environment_filename() -> Result<(), EnvmError> {
        let config = Config::from(r#"
            local = ".env"
            pattern = ".env.{}"
        "#)?;
        let env = Environment {
            config: &config,
            repo_path: Path::new("/repo"),
            head_path: Path::new("/repo/.envm/HEAD").to_path_buf(),
            backup_path: Path::new("/repo/.envm/.env.backup").to_path_buf(),
            current_env: EnvType::Local,
        };
        let filename = env.get_environment_filename("dev");
        assert_eq!(filename.as_path(), Path::new("/repo/.env.dev"));
        Ok(())
    }
}

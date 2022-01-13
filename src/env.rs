use std::{fs, fmt, path::{PathBuf, Path}};

use crate::error::EnvmError;
use crate::repository::Repository;
use crate::config::Config;

// To identify the current environment is local or other, since in most case we won't have another
// file for local environment, so we need to backup the environment file if we are using local
// environment configuration, then we can switch back to local environment later.
#[derive(Debug)]
enum Head {
    Local,
    Other(String),
}

impl Head {
    fn from(contents: &str) -> Head {
        let contents = contents.trim();
        if contents == "local" {
            Head::Local
        } else {
            Head::Other(String::from(contents))
        }
    }
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Head::Local => write!(f, "local"),
            Head::Other(value) => write!(f, "{}", value),
        }
    }
}

pub struct Environment<'a> {
    config: &'a Config,
    repo_path: &'a Path,
    head_path: PathBuf,
    head: Head,
}

impl<'a> Environment<'a> {
    pub fn new(repo: &Repository) -> Result<Environment, EnvmError> {
        let head_path = repo.envm_path().join("HEAD");
        let contents = fs::read_to_string(&head_path).map_err(|_| EnvmError::MissngHeadFile)?;
        let head = Head::from(&contents);
        Ok(Environment {
            config: repo.config(),
            repo_path: repo.path(),
            head_path,
            head,
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
        let head = Head::from(env);
        let data = format!("{}", head);
        fs::write(&self.head_path, data).unwrap();
    }

    pub fn use_environment(&self, env: &str) -> Result<(), EnvmError> {
        let target_env = self.get_environment_filename(env);
        let local_env = self.get_local_environment_filename();
        // TODO: backup env file if is local environment.
        if Path::new(&target_env).exists() {
            fs::copy(target_env, local_env).unwrap();
        } else {
            return Err(EnvmError::MissingTargetEnvironment(String::from(env)))
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
        let head = Head::from("local");
        assert!(matches!(head, Head::Local));
    }

    #[test]
    fn get_other_head() {
        let head = Head::from("dev");
        assert!(matches!(head, Head::Other(v) if v == "dev"));
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
            head: Head::Local,
        };
        let filename = env.get_environment_filename("dev");
        assert_eq!(filename.as_path(), Path::new("/repo/.env.dev"));
        Ok(())
    }
}

use std::{
    fs,
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
    pub fn new(path: PathBuf) -> Repository {
        Repository {
            path,
            config: Config::new(),
            current_env: EnvType::Local,
        }
    }

    pub fn load(path: PathBuf) -> Result<Repository, EnvmError> {
        let path = match lookup_repository(path) {
            Some(path) => path,
            None => return Err(EnvmError::NotEnvmRepository),
        };
        let config_path = path::get_config_path(&path);
        let contents = fs::read_to_string(config_path).map_err(|_| EnvmError::MissingConfigFile)?;
        let config = Config::from(&contents)?;
        let head_path = path::get_head_path(&path);
        let contents = fs::read_to_string(head_path).map_err(|_| EnvmError::MissngHeadFile)?;
        let current_env = EnvType::from(&contents);
        Ok(Repository {
            path,
            config,
            current_env,
        })
    }

    fn set_head(&self, env: &str) {
        let env = EnvType::from(env);
        let head_path = path::get_head_path(&self.path);
        fs::write(head_path, env.to_string()).unwrap();
    }

    pub fn use_environment(&self, env: &str) -> Result<(), EnvmError> {
        if env == self.current_env.to_string() {
            return Err(EnvmError::AlreadyUsingTargetEnvironment(String::from(env)));
        }
        let local_env_path = path::get_local_env_path(self);
        let backup_path = path::get_local_backup_path(self);
        if matches!(self.current_env, EnvType::Local) {
            fs::copy(&local_env_path, &backup_path)
                .map_err(|_| EnvmError::FailedToBackupLocalEnvironment)?;
        }

        let copy = |target, err| {
            if Path::new(target).exists() {
                fs::copy(target, local_env_path).unwrap();
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
                let target_env = path::get_env_path(self, env);
                copy(
                    &target_env,
                    EnvmError::MissingTargetEnvironment(String::from(env)),
                )?;
            }
        }
        self.set_head(env);
        Ok(())
    }

    pub fn init(&self) -> Result<PathBuf, EnvmError> {
        let envm_path = path::get_envm_path(&self.path);
        if envm_path.exists() {
            return Err(EnvmError::RepositoryAlreadyExists);
        }
        fs::create_dir(path::get_envm_path(&self.path)).unwrap();
        fs::write(path::get_config_path(&self.path), self.config.to_string()).unwrap();
        fs::write(
            path::get_head_path(&self.path),
            self.current_env.to_string(),
        )
        .unwrap();
        Ok(envm_path)
    }

    pub fn new_environment(&self, env: &str) -> Result<(), EnvmError> {
        let template_path = path::get_template_env_path(self);
        let target_path = match EnvType::from(env) {
            EnvType::Local => path::get_local_env_path(self),
            EnvType::Other(_) => path::get_env_path(self, env),
        };
        if !template_path.exists() {
            return Err(EnvmError::MissingTemplateEnvironment(
                self.config.template().clone(),
            ));
        }
        if target_path.exists() {
            return Err(EnvmError::TargetEnvironmentAlreadyExists(String::from(env)));
        }
        fs::copy(template_path, target_path).unwrap();
        Ok(())
    }
}

fn lookup_repository(dir: PathBuf) -> Option<PathBuf> {
    dir.ancestors()
        .take_while(|it| path::is_envm_repository(it))
        .next()
        .map(|it| it.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use tempfile;

    #[test]
    fn not_a_repository() {
        let root = Path::new("/not/a/repository").to_path_buf();
        assert_eq!(lookup_repository(root), None);
    }

    #[test]
    fn should_init_repo() -> Result<(), Box<dyn Error>> {
        let dir = tempfile::tempdir()?;
        let dir = dir.into_path();
        let repo = Repository::new(dir.clone());
        repo.init()?;
        assert!(path::get_envm_path(&repo.path).exists());
        assert!(path::get_config_path(&repo.path).exists());
        assert!(path::get_head_path(&repo.path).exists());
        fs::remove_dir_all(&dir)?;
        Ok(())
    }

    fn create_envm_repo_use_local_env() -> Result<Repository, Box<dyn Error>> {
        let dir = tempfile::tempdir()?;
        let dir = dir.into_path();
        let repo = Repository::new(dir.clone());
        repo.init()?;
        Ok(repo)
    }

    fn make_local_env_file(repo: &Repository) -> Result<PathBuf, Box<dyn Error>> {
        let path = path::get_local_env_path(repo);
        fs::write(&path, "ENV=local")?;
        Ok(path)
    }

    fn make_env_file(repo: &Repository, env: &str) -> Result<PathBuf, Box<dyn Error>> {
        let path = path::get_env_path(&repo, env);
        let data = format!("ENV={}", env);
        fs::write(&path, data)?;
        Ok(path)
    }

    #[test]
    fn should_use_target_env() -> Result<(), Box<dyn Error>> {
        let repo = create_envm_repo_use_local_env()?;
        let local_path = make_local_env_file(&repo)?;
        make_env_file(&repo, "dev")?;

        repo.use_environment("dev")?;

        let head_path = path::get_head_path(&repo.path);
        let head = fs::read_to_string(head_path)?;
        assert_eq!(head, EnvType::Other(String::from("dev")).to_string());

        let contents = fs::read_to_string(local_path)?;
        assert_eq!(contents, "ENV=dev");
        fs::remove_dir_all(repo.path)?;
        Ok(())
    }

    #[test]
    fn should_backup_local_env() -> Result<(), Box<dyn Error>> {
        let repo = create_envm_repo_use_local_env()?;
        make_local_env_file(&repo)?;
        make_env_file(&repo, "dev")?;

        repo.use_environment("dev")?;
        let backup_path = path::get_local_backup_path(&repo);
        assert!(backup_path.exists());
        let contents = fs::read_to_string(backup_path)?;
        assert_eq!(contents, "ENV=local");
        fs::remove_dir_all(repo.path)?;
        Ok(())
    }

    #[test]
    fn should_use_backup_local_env_use_local_env() -> Result<(), Box<dyn Error>> {
        let repo = create_envm_repo_use_local_env()?;
        let local_path = make_local_env_file(&repo)?;
        make_env_file(&repo, "dev")?;
        repo.use_environment("dev")?;
        let backup_path = path::get_local_backup_path(&repo);
        fs::write(&backup_path, "ENV=backup")?;

        // Sync repository state with file system
        let repo = Repository::load(repo.path)?;
        repo.use_environment("local")?;
        let contents = fs::read_to_string(local_path)?;
        assert_eq!(contents, "ENV=backup");
        fs::remove_dir_all(repo.path)?;
        Ok(())
    }

    fn make_template_env_file(repo: &Repository) -> Result<PathBuf, Box<dyn Error>> {
        let template_path = path::get_template_env_path(repo);
        fs::write(&template_path, "ENV=")?;
        Ok(template_path)
    }

    #[test]
    fn should_new_target_env_file() -> Result<(), Box<dyn Error>> {
        let repo = create_envm_repo_use_local_env()?;
        make_template_env_file(&repo)?;

        repo.new_environment("dev")?;

        let dev_path = path::get_env_path(&repo, "dev");
        assert!(dev_path.exists());
        fs::remove_dir_all(repo.path)?;
        Ok(())
    }

    #[test]
    fn should_new_local_env_file_by_config() -> Result<(), Box<dyn Error>> {
        let repo = create_envm_repo_use_local_env()?;
        make_template_env_file(&repo)?;

        repo.new_environment("local")?;

        let local_path = path::get_local_env_path(&repo);
        assert!(local_path.exists());
        fs::remove_dir_all(repo.path)?;
        Ok(())
    }
}

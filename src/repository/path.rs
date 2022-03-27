use crate::repository::Repository;
use std::path::{Component, Path, PathBuf};

pub fn get_config_path<P: AsRef<Path>>(path: P) -> PathBuf {
    get_envm_path(path).join("config")
}

pub fn get_current_path<P: AsRef<Path>>(path: P) -> PathBuf {
    get_envm_path(path).join("CURRENT")
}

// Ensure the env is a single text instead of path, so user can't access other file than
// environment file.
fn is_valid_env(env: &str) -> bool {
    let paths: Vec<Component> = Path::new(env).components().collect();
    paths.len() == 1
}

pub fn get_env_path(repo: &Repository, env: &str) -> PathBuf {
    if !is_valid_env(env) {
        panic!("The target environment is invalid: {}", env);
    }
    let filename = repo.config.pattern().replace("{}", env);
    repo.path.join(filename)
}

pub fn get_local_backup_path(repo: &Repository) -> PathBuf {
    get_envm_path(&repo.path).join(".env.backup")
}

pub fn get_local_env_path(repo: &Repository) -> PathBuf {
    repo.path.join(repo.config.local())
}

pub fn get_template_env_path(repo: &Repository) -> PathBuf {
    repo.path.join(repo.config.template())
}

pub fn get_envm_path<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref().join(".envm")
}

pub fn is_envm_repository<P: AsRef<Path>>(path: P) -> bool {
    get_envm_path(path).exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, fs};
    use tempfile;

    #[test]
    fn should_get_config_path() {
        let repo = Repository::new(Path::new("/repo").to_path_buf());
        assert_eq!(get_config_path(&repo.path), Path::new("/repo/.envm/config"));
    }

    #[test]
    fn should_get_current_path() {
        let repo = Repository::new(Path::new("/repo").to_path_buf());
        assert_eq!(get_current_path(&repo.path), Path::new("/repo/.envm/CURRENT"));
    }

    #[test]
    fn should_get_env_path() {
        let repo = Repository::new(Path::new("/repo").to_path_buf());
        let env = "dev";
        assert_eq!(get_env_path(&repo, env), Path::new("/repo/.env.dev"));
    }

    #[test]
    #[should_panic]
    fn should_not_get_env_path_out_of_repo() {
        let repo = Repository::new(Path::new("/repo").to_path_buf());
        let env = "dev/../../target.txt";
        get_env_path(&repo, env);
    }

    #[test]
    fn should_get_local_backup_path() {
        let repo = Repository::new(Path::new("/repo").to_path_buf());
        assert_eq!(
            get_local_backup_path(&repo),
            Path::new("/repo/.envm/.env.backup")
        );
    }

    #[test]
    fn should_get_envm_path() {
        let repo = Repository::new(Path::new("/repo").to_path_buf());
        assert_eq!(get_envm_path(&repo.path), Path::new("/repo/.envm"));
    }

    #[test]
    fn should_be_envm_repository() -> Result<(), Box<dyn Error>> {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join(".envm"))?;
        assert!(is_envm_repository(&dir));
        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn should_not_be_envm_repository() -> Result<(), Box<dyn Error>> {
        let dir = tempfile::tempdir().unwrap();
        assert!(!is_envm_repository(&dir));
        fs::remove_dir_all(dir)?;
        Ok(())
    }

    #[test]
    fn should_get_local_env_path() {
        let repo = Repository::new(Path::new("/repo").to_path_buf());
        assert_eq!(get_local_env_path(&repo), Path::new("/repo/.env"));
    }

    #[test]
    fn should_get_template_env_path() {
        let repo = Repository::new(Path::new("/repo").to_path_buf());
        assert_eq!(
            get_template_env_path(&repo),
            Path::new("/repo/.env.example")
        );
    }
}

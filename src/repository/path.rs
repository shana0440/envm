use std::path::{Path, PathBuf, Component};

pub fn get_config_path<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref().join(".envm").join("config")
}

pub fn get_head_path<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref().join(".envm").join("HEAD")
}

// Ensure the env is a single text instead of path, so user can't access other file than
// environment file.
fn is_valid_env(env: &str) -> bool {
    let paths: Vec<Component> = Path::new(env).components().collect();
    paths.len() == 1
}

pub fn get_env_path<P: AsRef<Path>>(path: P, pattern: &str, env: &str) -> PathBuf {
    if !is_valid_env(env) {
        panic!("The target environment is invalid: {}", env);
    }
    let filename = pattern.replace("{}", env);
    path.as_ref().join(filename)
}

pub fn get_local_backup_path<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref().join(".envm").join(".env.backup")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_config_path() {
        let path = Path::new("/repo");
        assert_eq!(get_config_path(path), Path::new("/repo/.envm/config"));
    }

    #[test]
    fn should_get_head_path() {
        let path = Path::new("/repo");
        assert_eq!(get_head_path(path), Path::new("/repo/.envm/HEAD"));
    }

    #[test]
    fn should_get_env_path() {
        let path = Path::new("/repo");
        let pattern = ".env.{}";
        let env = "dev";
        assert_eq!(get_env_path(path, pattern, env), Path::new("/repo/.env.dev"));
    }

    #[test]
    #[should_panic]
    fn should_not_get_env_path_out_of_repo() {
        let path = Path::new("/repo");
        let pattern = ".env.{}";
        let env = "dev/../../target.txt";
        get_env_path(path, pattern, env);
    }

    #[test]
    fn should_get_local_backup_path() {
        let path = Path::new("/repo");
        assert_eq!(get_local_backup_path(path), Path::new("/repo/.envm/.env.backup"));
    }
}

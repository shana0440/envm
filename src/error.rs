use std::{error::Error, fmt};

#[derive(Debug)]
pub enum EnvmError {
    NotEnvmRepository,
    MissingConfigFile,
    FailedToParseConfig,
    MissngHeadFile,
    MissingTargetEnvironment(String),
    FailedToBackupLocalEnvironment,
    MissingBackupEnvironment,
    RepositoryAlreadyExists,
    MissingTemplateEnvironment(String),
    TargetEnvironmentAlreadyExists(String),
    AlreadyUsingTargetEnvironment(String),
    RemovingUsingEnvironment(String),
    UnableReadGitignore(std::io::Error),
    UnableWriteGitignore(std::io::Error),
}

impl fmt::Display for EnvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            EnvmError::NotEnvmRepository => write!(
                f,
                "not a envm repository (or any of the parent directories)"
            ),
            EnvmError::MissingConfigFile => write!(f, "cannot found the configuration at .envm"),
            EnvmError::FailedToParseConfig => write!(f, "failed to parse the configuration"),
            EnvmError::MissngHeadFile => write!(f, "cannot found the head at .envm"),
            EnvmError::MissingTargetEnvironment(env) => {
                write!(f, "cannot found the {} environment", env)
            }
            EnvmError::FailedToBackupLocalEnvironment => {
                write!(f, "failed to backup the local environment")
            }
            EnvmError::MissingBackupEnvironment => {
                write!(f, "cannot found the backup local environment")
            }
            EnvmError::RepositoryAlreadyExists => write!(f, "the envm repository already exists"),
            EnvmError::MissingTemplateEnvironment(filename) => {
                write!(f, "cannot found the template environment: {}", filename)
            }
            EnvmError::TargetEnvironmentAlreadyExists(env) => {
                write!(f, "the '{}' environment already exists", env)
            }
            EnvmError::AlreadyUsingTargetEnvironment(env) => {
                write!(f, "already using '{}' environment", env)
            }
            EnvmError::RemovingUsingEnvironment(env) => {
                write!(f, "cannot remove current using environment: {}", env)
            }
            EnvmError::UnableReadGitignore(err) => {
                write!(f, "unable to read the .gitignore file: {}", err)
            }
            EnvmError::UnableWriteGitignore(err) => {
                write!(f, "unable to write to the .gitignore file: {}", err)
            }
        }
    }
}

impl Error for EnvmError {}

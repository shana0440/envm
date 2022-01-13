use std::fmt;

#[derive(Debug)]
pub enum EnvmError {
    NotEnvmRepository,
    MissingConfigFile,
    FailedToParseConfig,
}

impl fmt::Display for EnvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            EnvmError::NotEnvmRepository => write!(f, "not a envm repository (or any of the parent directories)"),
            EnvmError::MissingConfigFile => write!(f, "cannot found the configuration at .envm"),
            EnvmError::FailedToParseConfig => write!(f, "failed to parse the configuration"),
        }
    }
}

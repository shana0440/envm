// To identify the current environment is local or other, since in most case we won't have another
// file for local environment, so we need to backup the environment file if we are using local
// environment configuration, then we can switch back to local environment later.
#[derive(Debug)]
pub enum EnvType {
    Local,
    Other(String),
}

impl EnvType {
    pub fn from(contents: &str) -> EnvType {
        let contents = contents.trim();
        if contents == "local" {
            EnvType::Local
        } else {
            EnvType::Other(String::from(contents))
        }
    }

    pub fn to_string(&self) -> &str {
        match &self {
            EnvType::Local => "local",
            EnvType::Other(value) => &value,
        }
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
}

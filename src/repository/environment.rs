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

    pub fn is_equal(&self, target: &EnvType) -> bool {
        match target {
            EnvType::Local => match &self {
                EnvType::Local => true,
                EnvType::Other(_) => false,
            },
            EnvType::Other(target_str) => match &self {
                EnvType::Local => false,
                EnvType::Other(self_str) => target_str == self_str,
            },
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

    #[test]
    fn should_equal() {
        assert!(EnvType::from("local").is_equal(&EnvType::from("local")));
        assert!(EnvType::from("dev").is_equal(&EnvType::from("dev")));
    }

    #[test]
    fn should_not_equal() {
        assert!(!EnvType::from("local").is_equal(&EnvType::from("dev")));
        assert!(!EnvType::from("dev").is_equal(&EnvType::from("local")));
        assert!(!EnvType::from("dev").is_equal(&EnvType::from("production")));
    }
}

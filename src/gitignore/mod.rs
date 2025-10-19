use crate::error::EnvmError;
use std::path::PathBuf;

pub struct Gitignore {
    path: PathBuf,
    content: String,
}

impl Gitignore {
    pub fn load(path: PathBuf) -> Result<Self, EnvmError> {
        let content = if path.exists() {
            std::fs::read_to_string(&path).map_err(EnvmError::UnableReadGitignore)?
        } else {
            String::new()
        };

        Ok(Gitignore { path, content })
    }

    pub fn save(&self) -> Result<(), EnvmError> {
        std::fs::write(&self.path, &self.content).map_err(EnvmError::UnableWriteGitignore)
    }

    pub fn remove_pattern(&mut self, pattern: &str) -> &mut Self {
        let lines: Vec<String> = self
            .content
            .lines()
            .filter(|line| line.trim() != pattern.trim())
            .map(|line| line.to_string())
            .collect();

        self.content = lines.join("\n");
        if !self.content.is_empty() && !self.content.ends_with('\n') {
            self.content.push('\n');
        }
        self
    }

    pub fn ignore_patterns_section(&mut self, section: &str, patterns: Vec<String>) -> &mut Self {
        // Remove existing envm patterns first
        for pattern in &patterns {
            self.remove_pattern(pattern);
        }

        // Add proper spacing before envm section
        if !self.content.is_empty() {
            self.content.push('\n');
        }

        // Add envm section
        self.content.push_str(&format!("# {}\n", section));
        for pattern in patterns {
            self.content.push_str(&format!("{}\n", pattern));
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    fn create_temp_gitignore(content: &str) -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let gitignore_path = temp_dir.path().join(".gitignore");
        std::fs::write(&gitignore_path, content).unwrap();
        (temp_dir, gitignore_path)
    }

    #[test]
    fn test_ignore_section_patterns() {
        let content = "node_modules/\n*.log\n";
        let (_temp_dir, gitignore_path) = create_temp_gitignore(content);

        let mut gitignore = Gitignore::load(gitignore_path).unwrap();
        let patterns = vec![".envm".to_string(), ".env".to_string()];
        gitignore.ignore_patterns_section("envm", patterns);

        let result = gitignore.content;
        assert!(result.contains("# envm"));
        assert!(result.contains(".envm"));
        assert!(result.contains(".env"));
        assert!(result.starts_with("node_modules/\n*.log"));
    }

    #[test]
    fn test_ignore_section_patterns_removes_duplicates() {
        let content = "node_modules/\n.envm\n*.log\n.env\n";
        let (_temp_dir, gitignore_path) = create_temp_gitignore(content);

        let mut gitignore = Gitignore::load(gitignore_path).unwrap();
        let patterns = vec![".envm".to_string(), ".env".to_string()];
        gitignore.ignore_patterns_section("envm", patterns);

        let result = gitignore.content;

        // Should have only one instance of each pattern
        let envm_count = result.matches(".envm").count();
        assert_eq!(envm_count, 1);

        let env_lines = result.lines().filter(|line| line.trim() == ".env").count();
        assert_eq!(env_lines, 1);
    }
}

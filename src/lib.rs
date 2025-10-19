mod command;
mod configuration;
mod error;
mod gitignore;
mod repository;

use crate::command::{Command, UseCase};
use crate::error::EnvmError;
use crate::gitignore::Gitignore;
use crate::repository::Repository;
use colored::Colorize;
use std::env;

pub fn run() -> Result<(), EnvmError> {
    let current_dir = env::current_dir().unwrap();
    let command = Command::new();
    let use_case = command.run();
    match use_case {
        UseCase::InitConfiguration => {
            let repo = Repository::new(current_dir.clone());
            let repo_path = repo.init()?;
            println!("initialized envm repository in {}", repo_path.display());
        }
        other => {
            let repo = Repository::load(current_dir.clone())?;
            match other {
                UseCase::DiffEnvironment(target) => {
                    let (missing, extra) = repo.compare_to_template(&target);
                    if let Some(missing) = missing {
                        println!("missing variables:");
                        missing
                            .iter()
                            .map(|it| format!("- {}", it).red())
                            .for_each(|it| println!("{}", it));
                    }
                    if let Some(extra) = extra {
                        println!("extra variables:");
                        extra
                            .iter()
                            .map(|it| format!("+ {}", it).green())
                            .for_each(|it| println!("{}", it));
                    }
                }
                UseCase::UseEnvironment(target) => {
                    repo.use_environment(&target)?;
                    println!("switch to {} environment", target);
                }
                UseCase::NewEnvironment(env) => {
                    repo.new_environment(&env)?;
                    println!("create a new environment '{}'", env);
                }
                UseCase::ListEnvironments => {
                    for env in repo.list_environments() {
                        println!("{}", env);
                    }
                }
                UseCase::RemoveEnvironment(env) => {
                    repo.remove_environment(&env)?;
                    println!("removed environment '{}'", env);
                }
                UseCase::ShowCurrentUsingEnvironment => {
                    println!(
                        "currently using '{}' environment",
                        repo.current_env().to_string()
                    );
                }
                UseCase::Gitignore => {
                    let gitignore_path = current_dir.join(".gitignore");
                    let mut gitignore = Gitignore::load(gitignore_path)?;
                    let config = repo.config();
                    let gitignore_pattern = config.pattern().replace("{}", "*");
                    let env_template = format!("!{}", config.template());

                    let patterns = vec![
                        ".envm".to_string(),
                        config.local().clone(),
                        gitignore_pattern,
                        env_template,
                    ];
                    gitignore.ignore_patterns_section("envm", patterns);
                    gitignore.save()?;
                    println!("updated .gitignore with .envm and patterns from configuration");
                }
                _ => (),
            }
        }
    };
    Ok(())
}

mod command;
mod error;
mod repository;

use crate::command::{Command, UseCase};
use crate::error::EnvmError;
use crate::repository::Repository;
use std::env;

pub fn run() -> Result<(), EnvmError> {
    let current_dir = env::current_dir().unwrap();
    let command = Command::new();
    let use_case = command.run();
    match use_case {
        UseCase::InitConfiguration => {
            let repo = Repository::new(current_dir);
            let repo_path = repo.init()?;
            println!("initialized envm repository in {}", repo_path.display());
        }
        other => {
            let repo = Repository::load(current_dir)?;
            match other {
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
                _ => (),
            }
        }
    };
    Ok(())
}

mod command;
mod error;
mod repository;

use crate::command::{Command, UseCase};
use crate::error::EnvmError;
use crate::repository::Repository;

pub fn run() -> Result<(), EnvmError> {
    let command = Command::new();
    let use_case = command.run();
    match use_case {
        UseCase::InitConfiguration => {
            let repo = Repository::new();
            let repo_path = repo.init()?;
            println!("initialized envm repository in {}", repo_path.display());
        }
        other => {
            let repo = Repository::load()?;
            match other {
                UseCase::Use(target) => {
                    repo.use_environment(&target)?;
                    println!("switch to {} environment", target);
                }
                _ => (),
            }
        }
    };
    Ok(())
}

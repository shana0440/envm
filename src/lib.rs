mod command;
mod error;
mod repository;

use crate::command::{Command, UseCase};
use crate::error::EnvmError;
use crate::repository::Repository;

pub fn run() -> Result<(), EnvmError> {
    let command = Command::new();
    let repo = Repository::new()?;
    let use_case = command.run();
    match use_case {
        UseCase::Use(target) => {
            repo.use_environment(&target)?;
            println!("switch to {} environment", target);
        }
    };
    Ok(())
}

mod repository;
mod error;
mod command;

use crate::repository::Repository;
use crate::repository::environment::Environment;
use crate::error::EnvmError;
use crate::command::{Command, UseCase};

pub fn run() -> Result<(), EnvmError> {
    let command = Command::new();
    let repo = Repository::new()?;
    let env = Environment::new(&repo)?;
    let use_case = command.run();
    match use_case {
        UseCase::Use(target) => {
            env.use_environment(&target)?;
            println!("switch to {} environment", target);
        }
    };
    Ok(())
}

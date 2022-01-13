mod repository;
mod config;
mod error;
mod command;
mod env;

use crate::repository::Repository;
use crate::error::EnvmError;
use crate::command::{Command, UseCase};
use crate::env::Environment;

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

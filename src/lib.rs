mod repository;
mod config;
mod error;
mod command;

use crate::repository::Repository;
use crate::error::EnvmError;
use crate::command::{Command, UseCase};

pub fn run() -> Result<(), EnvmError> {
    let command = Command::new();
    let repo = Repository::new()?;
    let use_case = command.run();
    match use_case {
        UseCase::Use(env) => {
            println!("{}", env);
        }
    }
    Ok(())
}

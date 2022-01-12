mod repository;
mod config;
mod error;

use crate::repository::Repository;
use crate::error::EnvmError;

pub fn run() -> Result<(), EnvmError> {
    let repo = Repository::new()?;
    println!("the worktree folder is {}", repo.worktree().display());
    println!("the config is {:#?}", repo.config());
    Ok(())
}

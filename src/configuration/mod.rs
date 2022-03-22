use std::collections::HashMap;
use std::{fs, path::Path};

pub mod parser;

use crate::configuration::parser::dotenv;

pub type Configuration = HashMap<String, String>;

pub fn parse(path: &Path) -> Configuration {
    let content = fs::read_to_string(path).expect("Something went wrong reading the file");
    dotenv::parse(&content)
}

// This function return (missing, extra),
// the missing mean appear in left, but not appear in right
// extra mean appear in right, but not appear in left
pub fn compare<'a>(
    config_left: &'a Configuration,
    config_right: &'a Configuration,
) -> (Option<Vec<String>>, Option<Vec<String>>) {
    let missing: Vec<String> = config_left
        .keys()
        .filter(|it| !config_right.contains_key(it as &str))
        .map(|it| it.clone())
        .collect();
    let extra: Vec<String> = config_right
        .keys()
        .filter(|it| !config_left.contains_key(it as &str))
        .map(|it| it.clone())
        .collect();
    (
        match missing.len() {
            0 => None,
            _ => Some(missing),
        },
        match extra.len() {
            0 => None,
            _ => Some(extra),
        },
    )
}

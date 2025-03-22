use std::{env, path::PathBuf};

use crate::{
    feature::arg_feature::ArgFeature,
    parser::dir::parse_dir,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScanPath {
    pub value: PathBuf,
}

impl Default for ScanPath {
    fn default() -> Self {
        Self {
            value: env::current_dir()
                .unwrap_or_else(|_| env::home_dir().unwrap_or_default()),
        }
    }
}

impl ArgFeature for ScanPath {
    fn is_applicable(input: &str) -> Result<bool, &str> {
        match parse_dir(input) {
            Some(_) => Ok(true),
            None => Err(input),
        }
    }

    fn execute(input: Option<&str>) -> Self {
        let arg = match input {
            Some(x) => x,
            None => return Self::default(),
        };

        match parse_dir(arg) {
            Some(path) => Self { value: path },
            None => Self::default(),
        }
    }
}

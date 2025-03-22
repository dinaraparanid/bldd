use std::path::PathBuf;

use crate::{
    feature::arg_feature::ArgFeature,
    parser::{args::retrieve_args, dir::parse_dir},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Out {
    Console,
    Directory { value: PathBuf }
}

impl Default for Out {
    fn default() -> Self {
        Out::Console
    }
}

impl ArgFeature for Out {
    fn is_applicable(input: &str) -> Result<bool, &str> {
        let (cmd, path) = match retrieve_args(input) {
            Some(x) => x,
            None => return Ok(false),
        };

        if cmd != "-o" && cmd != "--out" {
            return Ok(false);
        }

        match parse_dir(path) {
            Some(_) => Ok(true),
            None => Err(path),
        }
    }

    fn execute(input: Option<&str>) -> Self {
        fn execute_impl(input: Option<&str>) -> Option<PathBuf> {
            let (_, path) = retrieve_args(input?)?;
            parse_dir(path)
        }

        match execute_impl(input) {
            Some(path) => Out::Directory { value: path },
            None => Out::Console,
        }
    }
}

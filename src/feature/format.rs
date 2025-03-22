use crate::{
    feature::arg_feature::ArgFeature,
    parser::args::retrieve_args,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    TXT,
    MD
}

impl Default for Format {
    fn default() -> Self {
        Format::TXT
    }
}

impl ArgFeature for Format {
    fn is_applicable(input: &str) -> Result<bool, &str> {
        let (cmd, fmt) = match retrieve_args(input) {
            Some(x) => x,
            None => return Ok(false),
        };

        if cmd != "-f" && cmd != "--format" {
            return Ok(false);
        }

        match fmt {
            "txt" => Ok(true),
            "md" => Ok(true),
            _ => Err(fmt),
        }
    }

    fn execute(input: Option<&str>) -> Self {
        fn execute_impl(input: Option<&str>) -> Option<Format> {
            let (_, fmt) = retrieve_args(input?)?;

            match fmt {
                "txt" => Some(Format::TXT),
                "md" => Some(Format::MD),
                _ => None
            }
        }

        execute_impl(input).unwrap_or_default()
    }
}

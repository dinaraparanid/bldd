use crate::feature::arg_feature::ArgFeature;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Help {}

impl ArgFeature for Help {
    fn is_applicable(input: &str) -> Result<bool, &str> {
        let arg = input.trim();
        Ok(arg == "--help" || arg == "-h")
    }

    fn execute(_: Option<&str>) -> Self {
        println!("
        Usage: blld <options> /path/to/your/directory

        Options:
            -h --help               Display this information
            -o --out[=/path/]       Output path (console if not specified)
            -f --format[=txt;md]    Output format (.txt if not specified)
        ");

        Self {}
    }
}

pub trait ArgFeature {
    fn is_applicable(input: &str) -> Result<bool, &str>;
    fn execute(input: Option<&str>) -> Self;
}

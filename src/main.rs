use linux_lab1::{executor::{file_walker::walk, report_generator::generate_report}, feature::{arg_feature::ArgFeature, help::Help}, parser::feature_parser};

fn main() {
    app().unwrap_or_else(|_| {
        Help::execute(None);
    })
}

#[inline]
fn app() -> Result<(), ()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let features = feature_parser::parse_features(args)?;

    println!("Beginning generating report. It may take a while, please stand by...");

    let output = walk(&features);
    generate_report(features, output);
    Ok(())
}

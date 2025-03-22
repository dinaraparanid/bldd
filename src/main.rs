use linux_lab1::{executor::file_walker::walk, parser::feature_parser};

fn main() -> Result<(), ()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let features = feature_parser::parse_features(args)?;
    let result = walk(&features);
    println!("OUT {:?}", result);
    Ok(())
}

use linux_lab1::parser::feature_parser;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let features = feature_parser::parse_features(args);
    println!("{:?}", features);
}

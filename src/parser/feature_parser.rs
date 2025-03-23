use crate::{
    feature::{
        arg_feature::ArgFeature, feature_builder::{AppFeatureBuilder, AppFeatures},
        format::Format,
        help::Help,
        out::Out,
        scan_path::ScanPath,
    },
    parser::fatal_error::FatalError,
};

pub fn parse_features(args: Vec<String>) -> Result<AppFeatures, ()> {
    args
        .into_iter()
        .fold(Ok(AppFeatures::builder()), |builder_res, arg| {
            builder_res.map(|builder| find_feature(arg, builder))?
        })
        .map(|builder| builder.build())
}

fn find_feature(arg: String, builder: AppFeatureBuilder) -> Result<AppFeatureBuilder, ()> {
    if Help::is_applicable(&arg).unwrap() {
        return Err(())
    }

    if Format::is_applicable(&arg).get_or_exit(|arg| format!("Invalid format: {}", arg)) {
        return Ok(builder.set_format(Format::execute(Some(&arg))));
    }

    if Out::is_applicable(&arg).get_or_exit(|arg| format!("Invalid directory: {}", arg)) {
        return Ok(builder.set_out(Out::execute(Some(&arg))));
    }

    if ScanPath::is_applicable(&arg).get_or_exit(|arg| format!("Invalid directory: {}", arg)) {
        return Ok(builder.set_path(ScanPath::execute(Some(&arg))));
    }

    Err(())
}

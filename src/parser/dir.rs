use std::path::{Path, PathBuf};

pub fn parse_dir(path: &str) -> Option<PathBuf> {
    match std::fs::metadata(path) {
        Ok(meta) => if meta.is_dir() { Some(Path::new(path).to_path_buf()) } else { None },
        Err(_) => None,
    }
}

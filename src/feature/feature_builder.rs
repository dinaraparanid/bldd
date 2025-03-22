use crate::feature::{format::Format, out::Out, scan_path::ScanPath};

#[derive(Clone, Debug, Default)]
pub struct AppFeatures {
    format: Format,
    out: Out,
    path: ScanPath,
}

#[derive(Clone, Debug, Default)]
pub struct AppFeatureBuilder {
    format: Format,
    out: Out,
    path: ScanPath,
}

impl AppFeatureBuilder {
    pub fn set_format(self, format: Format) -> Self {
        Self {
            format: format,
            out: self.out,
            path: self.path,
        }
    }

    pub fn set_out(self, out: Out) -> Self {
        Self {
            format: self.format,
            out: out,
            path: self.path,
        }
    }

    pub fn set_path(self, path: ScanPath) -> Self {
        Self {
            format: self.format,
            out: self.out,
            path: path,
        }
    }

    pub fn build(self) -> AppFeatures {
        AppFeatures {
            format: self.format,
            out: self.out,
            path: self.path,
        }
    }
}

impl AppFeatures {
    pub fn builder() -> AppFeatureBuilder {
        AppFeatureBuilder::default()
    }
}

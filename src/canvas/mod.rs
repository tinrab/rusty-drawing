use serde::{Deserialize, Serialize};

use crate::canvas::path::Path;

pub mod color;
pub mod line;
pub mod path;
pub mod point;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas {
    paths: Vec<Path>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas { paths: Vec::new() }
    }

    pub fn add_path(&mut self, path: Path) {
        self.paths.push(path);
    }
}

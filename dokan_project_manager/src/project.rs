use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Project {
    pub(crate) path: PathBuf,
}

impl Project {
    pub fn get_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    pub fn get_path(&self) -> &str {
        self.path.to_str().unwrap()
    }
}

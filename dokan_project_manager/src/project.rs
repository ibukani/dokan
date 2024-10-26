use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Project {
    pub(crate) path: PathBuf,
    pub(crate) timestamp: DateTime<Utc>
}

impl Project {
    pub fn get_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    pub fn get_path(&self) -> &str {
        self.path.to_str().unwrap()
    }

    pub fn get_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub(crate) fn update_timestamp(&mut self) {
        self.timestamp = Utc::now();
    }
}

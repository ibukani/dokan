use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Project {
    pub(crate) path: String,
}

impl Project {
    pub fn get_name(&self) -> &str {
        self.path.split('/').last().unwrap()
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

use std::path::PathBuf;

pub struct PathResolver {
    current_path: PathBuf,
}

impl PathResolver {
    pub fn new() -> Self {
        Self {
            current_path: std::env::current_dir().unwrap(),
        }
    }

    pub fn resolve(&mut self, path: PathBuf) -> PathBuf {
        if path.is_relative() {
            self.current_path.join(path)
        } else {
            path
        }
    }
}
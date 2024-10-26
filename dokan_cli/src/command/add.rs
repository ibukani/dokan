use crate::util::path_resolver::PathResolver;
use normpath::PathExt;
use std::path::PathBuf;
use dokan_project_manager::add_project;

pub fn add(path_buf: PathBuf) {
    let path_buf = PathResolver::new().resolve(path_buf);
    let normalized_path = match path_buf.normalize() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to normalize path: {}", e);
            return;
        }
    };

    if !normalized_path.exists() {
        eprintln!("Path does not exist: {}", normalized_path.as_path().to_str().unwrap());
        return;
    }

    add_project(path_buf.to_str().unwrap());
    println!("Project added successfully. path: {}", normalized_path.as_path().to_str().unwrap());
}
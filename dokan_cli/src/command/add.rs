use crate::util::path_resolver::PathResolver;
use dokan_project_manager::add_project;
use normpath::PathExt;
use std::path::PathBuf;

pub fn add(path_buf: PathBuf) {
    let path_buf = PathResolver::new().resolve(path_buf);
    let normalized_path = match path_buf.normalize() {
        Ok(path) => path.into_path_buf(),
        Err(e) => {
            eprintln!("Failed to normalize path: {}", e);
            return;
        }
    };

    if !normalized_path.exists() {
        eprintln!(
            "Path does not exist: {}",
            normalized_path.as_path().to_str().unwrap()
        );
        return;
    }

    match add_project(normalized_path) {
        Ok(project) => {
            println!("Project added successfully. path: {}", project.get_path());
        }
        Err(e) => {
            eprintln!("Failed to add project: {:?}", e);
        }
    }
}

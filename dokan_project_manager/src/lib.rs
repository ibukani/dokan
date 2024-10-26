mod config;
mod project;

use crate::config::ConfigFile;
use crate::project::Project;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ProjectAddError {
    AlreadyExists,
}

pub fn project_list() -> Vec<Project> {
    let config_file = ConfigFile::load_or_create(Default::default());
    config_file.data.get_projects()
}

pub fn add_project(file_path: PathBuf) -> Result<Project, ProjectAddError> {
    let mut config_file = ConfigFile::load_or_create(Default::default());
    let project = Project { path: file_path };

    // check if project already exists
    if config_file.data.project_exists(project.get_name()) {
        return Err(ProjectAddError::AlreadyExists);
    }

    config_file.data.add_project(project.clone());
    config_file.save();

    Ok(project)
}

pub fn remove_project(project_name: &str) {
    let mut config_file = ConfigFile::load_or_create(Default::default());
    config_file.data.remove_project(project_name);
    config_file.save();
}

mod config;
mod project;

use crate::config::ConfigFile;
use crate::project::Project;
use std::path::PathBuf;

pub fn project_list() -> Vec<Project> {
    let config_file = ConfigFile::load_or_create(Default::default());
    config_file.data.get_projects()
}

pub fn add_project(file_path: PathBuf) -> Project {
    let mut config_file = ConfigFile::load_or_create(Default::default());
    let project = Project { path: file_path };

    config_file.data.add_project(project.clone());
    config_file.save();

    project
}

pub fn remove_project(project_name: &str) {
    let mut config_file = ConfigFile::load_or_create(Default::default());
    config_file.data.remove_project(project_name);
    config_file.save();
}

mod config;
mod project;

use crate::project::Project;
use serde::{Deserialize, Serialize};
use crate::config::ConfigFile;

pub fn project_list() -> Vec<Project> {
    let config_file = ConfigFile::load_or_create(Default::default());
    config_file.data.get_projects()
}

pub fn add_project(file_path: &str) {
    let mut config_file = ConfigFile::load_or_create(Default::default());
    config_file.data.add_project(Project {
        path: file_path.to_string(),
    });
    config_file.save();
}

pub fn remove_project(project_name: &str) {
    let mut config_file = ConfigFile::load_or_create(Default::default());
    config_file.data.remove_project(project_name);
    config_file.save();
}

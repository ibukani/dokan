mod config;
pub mod project;
mod project_list;

use crate::config::ConfigFile;
use crate::project::Project;
use std::path::PathBuf;
use chrono::Utc;

#[derive(Debug)]
pub enum ProjectAddError {
    AlreadyExists,
}

#[derive(Debug)]
pub enum ProjectRemoveError {
    NotFound,
}

pub fn project_list() -> Vec<Project> {
    let config_file = ConfigFile::load_or_create(Default::default());
    config_file.data.get_project_list().get_projects()
}

pub fn update_project_timestamp(project_name: &str) -> Result<(), ProjectRemoveError> {
    let mut config_file = ConfigFile::load_or_create(Default::default());
    let project_list = config_file.data.get_project_list_mut();

    // check if project exists
    if !project_list.project_exists(project_name) {
        return Err(ProjectRemoveError::NotFound);
    }

    project_list.update_project_timestamp(project_name);
    config_file.save();

    Ok(())
}

pub fn get_project(project_name: &str) -> Option<Project> {
    let config_file = ConfigFile::load_or_create(Default::default());
    let project_list = config_file.data.get_project_list();

    project_list.get_project(project_name).cloned()
}

pub fn add_project(file_path: PathBuf) -> Result<Project, ProjectAddError> {
    let mut config_file = ConfigFile::load_or_create(Default::default());
    let project_list = config_file.data.get_project_list_mut();
    let project = Project { path: file_path, timestamp: Utc::now() };

    // check if project already exists
    if project_list.project_exists(project.get_name()) {
        return Err(ProjectAddError::AlreadyExists);
    }

    project_list.add_project(project.clone());
    config_file.save();

    Ok(project)
}

pub fn remove_project(project_name: &str) -> Result<(), ProjectRemoveError> {
    let mut config_file = ConfigFile::load_or_create(Default::default());
    let project_list = config_file.data.get_project_list_mut();

    // check if project exists
    if !project_list.project_exists(project_name) {
        return Err(ProjectRemoveError::NotFound);
    }

    project_list.remove_project(project_name);
    config_file.save();

    Ok(())
}

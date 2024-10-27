use crate::project::Project;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ProjectList {
    projects: HashMap<String, Project>,
}

impl ProjectList {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    pub fn get_project(&self, project_name: &str) -> Option<&Project> {
        self.projects.get(project_name)
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.insert(project.get_name().to_string(), project);
    }

    pub fn remove_project(&mut self, project_name: &str) {
        self.projects.remove(project_name);
    }

    pub fn project_exists(&self, project_name: &str) -> bool {
        self.projects.contains_key(project_name)
    }

    pub fn update_project_timestamp(&mut self, project_name: &str) {
        self.projects
            .get_mut(project_name)
            .expect("project not found")
            .update_timestamp();
    }

    pub fn get_projects(&self) -> Vec<Project> {
        self.projects.values().cloned().collect()
    }
}
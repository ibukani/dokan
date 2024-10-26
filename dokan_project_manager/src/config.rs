use std::collections::HashMap;
use crate::Project;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::option::Option;
use std::path::PathBuf;

pub struct ConfigFile {
    path: PathBuf,
    pub data: ConfigData
}

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigData {
    projects: Option<HashMap<String, Project>>
}

impl ConfigFile {
    pub fn load_or_create(default_data: ConfigData) -> Self {
        let config_path = get_config_path()
            .expect("Cannot find config folder")
            .join("config.toml");

        match fs::read_to_string(&config_path) {
            Ok(file_data) => {
                let loaded_data :ConfigData =
                    match toml::from_str(file_data.as_str()) {
                        Ok(data) => { data }
                        Err(_) => { default_data }
                    };

                ConfigFile {
                    path: config_path,
                    data: loaded_data,
                }
            }
            Err(_) => {
                // create file
                if let Some(parent) = config_path.parent() {
                    fs::create_dir_all(parent)
                        .expect("cannot create dir");
                }
                let mut config_file = File::create(&config_path)
                    .expect("cannot create config file");
                // write file
                config_file
                    .write_all(
                        toml::to_string(&default_data).unwrap().as_bytes()
                    )
                    .expect("cannot write config file");

                ConfigFile {
                    path: config_path,
                    data: default_data,
                }
            }
        }
    }

    pub fn save(&self) {
        // open file
        let config_path = &self.path;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .expect("cannot create dir");
        }
        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_path)
            .expect("cannot open directory");

        // write
        config_file.write_all(toml::to_string(&self.data).unwrap().as_bytes())
            .expect("cannot write config file");
    }
}

impl ConfigData {
    pub fn get_projects(&self) -> Vec<Project> {
        self.projects.clone()
            .unwrap_or_default()
            .values()
            .map(|project| project.clone())
            .collect()
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.get_or_insert(HashMap::new())
            .insert(project.get_name().to_string(), project);
    }

    pub fn remove_project(&mut self, project_name: &str) {
        self.projects.get_or_insert(HashMap::new())
            .remove(project_name);
    }
}

fn get_config_path() -> Result<PathBuf, PathBuf> {
    match std::env::var("XDG_CONFIG_HOME") {
        Ok(path) => {
            Ok(PathBuf::from(path).join("dokan"))
        }
        Err(_) => {
            if let Some(proj_dirs) = ProjectDirs::from("", "", "dokan") {
                Ok(proj_dirs.config_dir().to_path_buf())
            } else {
                Err(PathBuf::default())
            }
        }
    }
}
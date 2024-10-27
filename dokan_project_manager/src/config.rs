use crate::project_list::ProjectList;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::option::Option;
use std::path::PathBuf;

pub struct ConfigFile {
    path: PathBuf,
    pub data: SettingData,
}

pub struct SettingData {
    project_list: ProjectList,
}

#[derive(Serialize, Deserialize, Default)]
pub struct FileData {
    project_list: Option<ProjectList>,
}

impl ConfigFile {
    pub fn load_or_create(default_data: FileData) -> Self {
        let config_path = get_config_path()
            .expect("Cannot find config folder")
            .join("config.toml");

        match fs::read_to_string(&config_path) {
            Ok(file_data) => {
                let loaded_data= FileData::parse(&file_data, default_data);

                ConfigFile {
                    path: config_path,
                    data: SettingData::from(loaded_data),
                }
            }
            Err(_) => {
                // create file
                if let Some(parent) = config_path.parent() {
                    fs::create_dir_all(parent).expect("cannot create dir");
                }
                let mut config_file =
                    File::create(&config_path).expect("cannot create config file");
                // write file
                config_file
                    .write_all(toml::to_string(&default_data).unwrap().as_bytes())
                    .expect("cannot write config file");

                ConfigFile {
                    path: config_path,
                    data: SettingData::from(default_data),
                }
            }
        }
    }

    pub fn save(&self) {
        // open file
        let config_path = &self.path;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).expect("cannot create dir");
        }
        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_path)
            .expect("cannot open directory");

        // write
        config_file
            .write_all(self.data.to_file_data().to_string().as_bytes())
            .expect("cannot write config file");
    }
}

impl SettingData {
    fn from(file_data: FileData) -> Self {
        SettingData {
            project_list: file_data.project_list.unwrap_or_else(|| ProjectList::new()),
        }
    }

    fn to_file_data(&self) -> FileData {
        FileData {
            project_list: Some(self.project_list.clone()),
        }
    }

    pub fn get_project_list(&self) -> &ProjectList {
        &self.project_list
    }

    pub fn get_project_list_mut(&mut self) -> &mut ProjectList {
        &mut self.project_list
    }
}

impl FileData {
    fn parse(file_data: &str, default_data: FileData) -> Self {
        toml::from_str(file_data).unwrap_or_else(|_| default_data)
    }

    fn to_string(&self) -> String {
        toml::to_string(self).unwrap()
    }
}

fn get_config_path() -> Result<PathBuf, PathBuf> {
    match std::env::var("XDG_CONFIG_HOME") {
        Ok(path) => Ok(PathBuf::from(path).join("dokan")),
        Err(_) => {
            if let Some(proj_dirs) = ProjectDirs::from("", "", "dokan") {
                Ok(proj_dirs.config_dir().to_path_buf())
            } else {
                Err(PathBuf::default())
            }
        }
    }
}

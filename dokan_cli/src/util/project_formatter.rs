use anstyle::AnsiColor;
use chrono::Utc;
use dokan_project_manager::project::Project;

#[derive(Debug)]
pub struct ViewSetting {
    pub(crate) name: bool,
    pub(crate) folder: bool,
    pub(crate) timestamp: bool,
}

impl Default for ViewSetting {
    fn default() -> Self {
        Self {
            name: true,
            folder: true,
            timestamp: false,
        }
    }
}

pub fn format_project(project: &Project, setting: ViewSetting) -> String {
    let mut result = String::new();

    if setting.name {
        result.push_str(&format!("{}", project.get_name()));
    }

    if setting.folder {
        let style = anstyle::Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
        result.push_str(&format!("{style}(\u{e613} {})", project.get_path()));
    }

    if setting.timestamp {
        let style = anstyle::Style::new().fg_color(Some(AnsiColor::Yellow.into()));
        result.push_str(&format!("{style} - {}", Utc::now()));
    }

    result
}
use crate::command::ListArgs;
use crate::util::project_formatter::{format_project, ViewSetting};

pub fn list(args: ListArgs) {
    let projects = dokan_project_manager::project_list();

    for project in projects {
        println!("{}", format_project(&project, Default::default()));
    }
}

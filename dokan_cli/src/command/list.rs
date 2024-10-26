use crate::command::ListArgs;
use crate::util::project_formatter::{format_project, ViewSetting};

pub(crate) fn list(args: ListArgs) {
    let projects = dokan_project_manager::project_list();
    let setting = ViewSetting {
        name: args.all || args.name,
        path: args.all || args.path,
        timestamp: args.all || args.timestamp,
    };

    println!("Projects:");
    for project in projects {
        println!("- {}", format_project(&project, setting));
    }
}

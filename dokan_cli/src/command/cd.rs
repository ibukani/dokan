use crate::command::CdArgs;
use std::path::Path;

pub fn cd(args: CdArgs) {
    let project = match dokan_project_manager::get_project(&args.name) {
        Some(project) => project,
        None => {
            eprintln!("Project not found");
            return;
        }
    };

    let path = Path::new(project.get_path()).join("src");

    if !path.exists() {
        eprintln!("Path does not exist: {}", path.display());
        return;
    }
}
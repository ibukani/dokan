pub fn remove(project_name: String) {
    match dokan_project_manager::remove_project(&project_name) {
        Ok(_) => {
            println!("Project {} removed", project_name);
        }
        Err(dokan_project_manager::ProjectRemoveError::NotFound) => {
            println!("Project {} not found", project_name);
        }
    }
}

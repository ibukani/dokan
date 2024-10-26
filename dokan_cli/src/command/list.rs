pub fn list() {
    let projects = dokan_project_manager::project_list();
    for project in projects {
        println!("{}({})", project.get_name(), project.get_path());
    }
}

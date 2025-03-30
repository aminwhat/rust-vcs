use std::fs;
use std::path::Path;

pub fn init_repo() {
    let vcs_dir = ".rustvcs";
    let commits_dir = format!("{}/commits", vcs_dir);
    let branches_dir = format!("{}/branches", vcs_dir);
    let head_path = format!("{}/HEAD", vcs_dir);
    let main_branch_path = format!("{}/main.json", branches_dir);

    if Path::new(vcs_dir).exists() {
        println!("Repository already initialized.");
        return;
    }

    fs::create_dir_all(&commits_dir).unwrap();
    fs::create_dir_all(&branches_dir).unwrap();
    fs::write(&head_path, "main").unwrap(); // Initialize HEAD to 'main'

    println!("Initialized empty RustVCS repository in {}", vcs_dir);
}

use std::fs;
use std::path::Path;

pub fn create_branch(branch_name: &str) {
    let vcs_dir = ".rustvcs";
    let head_path = format!("{}/HEAD", vcs_dir);
    let branch_path = format!("{}/branches/{}.json", vcs_dir, branch_name);

    if !Path::new(&head_path).exists() {
        println!("No repository found. Run `init` first.");
        return;
    }

    if Path::new(&branch_path).exists() {
        println!("Branch '{}' already exists.", branch_name);
        return;
    }

    let current_branch = fs::read_to_string(&head_path).unwrap_or("main".to_string());
    let current_branch_path = format!("{}/branches/{}.json", vcs_dir, current_branch);

    if !Path::new(&current_branch_path).exists() {
        println!("Current branch has no commits yet.");
        return;
    }

    let latest_commit = fs::read_to_string(&current_branch_path).unwrap();
    fs::write(&branch_path, &latest_commit).unwrap();
    fs::write(&head_path, branch_name).unwrap(); // Switch HEAD to new branch
    println!("Switched to new branch '{}'.", branch_name);
}

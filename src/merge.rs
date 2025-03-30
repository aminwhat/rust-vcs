use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn merge_branch(target_branch: &str) {
    let vcs_dir = ".rustvcs";
    let head_path = format!("{}/HEAD", vcs_dir);
    let current_branch = fs::read_to_string(&head_path).unwrap_or("main".to_string());

    if current_branch == target_branch {
        println!("Already on '{}'. Nothing to merge.", target_branch);
        return;
    }

    let target_branch_path = format!("{}/branches/{}.json", vcs_dir, target_branch);
    let current_branch_path = format!("{}/branches/{}.json", vcs_dir, current_branch);

    if !Path::new(&target_branch_path).exists() {
        println!("Branch '{}' does not exist.", target_branch);
        return;
    }

    // Load latest commits of both branches
    let target_commit: Value =
        serde_json::from_str(&fs::read_to_string(&target_branch_path).unwrap()).unwrap();
    let mut current_commit: Value =
        serde_json::from_str(&fs::read_to_string(&current_branch_path).unwrap()).unwrap();

    let target_files = target_commit["files"].as_object().unwrap();
    let current_files = current_commit["files"].as_object_mut().unwrap();

    // Simple merge: copy new/updated files from target to current
    for (file, content) in target_files {
        current_files.insert(file.clone(), content.clone());
    }

    // Save merged commit
    let new_commit_path = format!(
        "{}/commits/{}.json",
        vcs_dir,
        current_commit["id"].as_str().unwrap()
    );
    fs::write(
        new_commit_path,
        serde_json::to_string_pretty(&current_commit).unwrap(),
    )
    .unwrap();
    fs::write(
        current_branch_path,
        serde_json::to_string_pretty(&current_commit).unwrap(),
    )
    .unwrap();

    println!("Merged '{}' into '{}'.", target_branch, current_branch);
}

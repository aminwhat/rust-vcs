use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn commit_changes(message: &str) {
    let vcs_dir = ".rustvcs";
    let head_path = format!("{}/HEAD", vcs_dir);
    let commits_dir = format!("{}/commits", vcs_dir);
    let branches_dir = format!("{}/branches", vcs_dir);

    if !Path::new(&head_path).exists() {
        println!("No repository found. Run `init` first.");
        return;
    }

    let branch = fs::read_to_string(&head_path).unwrap_or("main".to_string());
    let branch_path = format!("{}/{}.json", branches_dir, branch);

    // Create commit ID (for simplicity, using timestamp)
    let commit_id = format!("{}", chrono::Utc::now().timestamp());
    let commit_path = format!("{}/{}.json", commits_dir, commit_id);

    // Collect all tracked files
    let mut files = HashMap::new();
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() != "json" {
            let content = fs::read_to_string(&path).unwrap();
            files.insert(path.to_str().unwrap().to_string(), content);
        }
    }

    let commit_data = json!({
        "id": commit_id,
        "message": message,
        "files": files
    });

    fs::write(
        &commit_path,
        serde_json::to_string_pretty(&commit_data).unwrap(),
    )
    .unwrap();
    fs::write(
        &branch_path,
        serde_json::to_string_pretty(&commit_data).unwrap(),
    )
    .unwrap();

    println!("Committed as {}", commit_id);
}

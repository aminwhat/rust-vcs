use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn commit_changes(message: &str) {
    let vcs_dir = ".rustvcs";
    let staging_path = format!("{}/staging.json", vcs_dir);
    let commits_dir = format!("{}/commits", vcs_dir);
    let branches_dir = format!("{}/branches", vcs_dir);
    let head_path = format!("{}/HEAD", vcs_dir);

    if !Path::new(&staging_path).exists() {
        println!("No files staged. Use `add` to stage files.");
        return;
    }

    // Read the current branch from HEAD
    let branch = fs::read_to_string(&head_path).unwrap_or("main".to_string());
    let branch_path = format!("{}/{}.json", branches_dir, branch);

    // Create commit ID (for simplicity, using timestamp)
    let commit_id = format!("{}", chrono::Utc::now().timestamp());
    let commit_path = format!("{}/{}.json", commits_dir, commit_id);

    // Load staged files
    let staged_files = fs::read_to_string(&staging_path).unwrap();
    let staged_files: HashMap<String, String> = serde_json::from_str(&staged_files).unwrap();

    // Get current timestamp in ISO 8601 format
    let timestamp = chrono::Utc::now().to_rfc3339();

    // Prepare the commit data
    let commit_data = json!({
        "id": commit_id,
        "message": message,
        "timestamp": timestamp,
        "files": staged_files
    });

    // Save the commit
    fs::write(
        &commit_path,
        serde_json::to_string_pretty(&commit_data).unwrap(),
    )
    .unwrap();

    // Update the current branch with the latest commit
    let mut branch_data: HashMap<String, serde_json::Value> = if Path::new(&branch_path).exists() {
        let content = fs::read_to_string(&branch_path).unwrap();
        serde_json::from_str(&content).unwrap()
    } else {
        HashMap::new()
    };

    let commits = branch_data
        .entry("commits".to_string())
        .or_insert_with(|| json!([]));
    commits.as_array_mut().unwrap().push(commit_data);

    fs::write(
        &branch_path,
        serde_json::to_string_pretty(&branch_data).unwrap(),
    )
    .unwrap();

    // Clear staging area after commit
    fs::write(&staging_path, "{}").unwrap();

    println!("Committed as {} to branch '{}'", commit_id, branch);
}

use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn checkout(commit_id: &str) {
    let commit_path = format!(".rustvcs/commits/{}.json", commit_id);
    if !Path::new(&commit_path).exists() {
        println!("Commit not found.");
        return;
    }

    let content = fs::read_to_string(&commit_path).unwrap();
    let commit: Value = serde_json::from_str(&content).unwrap();

    for (filename, file_content) in commit["files"].as_object().unwrap() {
        fs::write(filename, file_content.as_str().unwrap()).unwrap();
    }

    println!("Checked out commit {}", commit_id);
}

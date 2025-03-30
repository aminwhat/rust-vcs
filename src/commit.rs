use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn commit_changes(message: &str) {
    let vcs_dir = ".rustvcs";
    let index_path = format!("{}/index.json", vcs_dir);
    let commits_dir = format!("{}/commits", vcs_dir);

    if !Path::new(&index_path).exists() {
        println!("No changes to commit.");
        return;
    }

    let index: HashMap<String, String> =
        serde_json::from_str(&fs::read_to_string(&index_path).unwrap_or("{}".to_string())).unwrap();

    if index.is_empty() {
        println!("No files in staging.");
        return;
    }

    let commit_id = Uuid::new_v4().to_string();
    let commit_data = json!({
        "id": commit_id,
        "timestamp": Utc::now().to_rfc3339(),
        "message": message,
        "files": index
    });

    fs::write(
        format!("{}/{}.json", commits_dir, commit_id),
        serde_json::to_string_pretty(&commit_data).unwrap(),
    )
    .unwrap();
    fs::write(index_path, "{}").unwrap(); // Clear staging

    println!("Committed changes: {}", commit_id);
}

use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn show_log() {
    let vcs_dir = ".rustvcs/commits";

    if !Path::new(vcs_dir).exists() {
        println!("No commit history.");
        return;
    }

    let entries = fs::read_dir(vcs_dir).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() {
            let content = fs::read_to_string(&path).unwrap();
            let commit: Value = serde_json::from_str(&content).unwrap();
            println!(
                "Commit {}: {} at {}",
                commit["id"], commit["message"], commit["timestamp"]
            );
        }
    }
}

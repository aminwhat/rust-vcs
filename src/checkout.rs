use std::fs;
use std::path::Path;

pub fn checkout(branch_or_commit: &str) {
    let vcs_dir = ".rustvcs";
    let head_path = format!("{}/HEAD", vcs_dir);
    let branch_path = format!("{}/branches/{}.json", vcs_dir, branch_or_commit);
    let commit_path = format!("{}/commits/{}.json", vcs_dir, branch_or_commit);

    if Path::new(&branch_path).exists() {
        fs::write(&head_path, branch_or_commit).unwrap(); // Update HEAD
        println!("Switched to branch '{}'.", branch_or_commit);
    } else if Path::new(&commit_path).exists() {
        let content = fs::read_to_string(&commit_path).unwrap();
        let commit: serde_json::Value = serde_json::from_str(&content).unwrap();

        for (filename, file_content) in commit["files"].as_object().unwrap() {
            fs::write(filename, file_content.as_str().unwrap()).unwrap();
        }

        println!("Checked out commit {}", branch_or_commit);
    } else {
        println!("Branch or commit '{}' not found.", branch_or_commit);
    }
}

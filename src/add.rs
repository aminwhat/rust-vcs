use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn add_file(path: &str) {
    let vcs_dir = ".rustvcs";
    let staging_path = format!("{}/staging.json", vcs_dir);

    if !Path::new(vcs_dir).exists() {
        println!("No repository found. Run `init` first.");
        return;
    }

    let mut staged_files = if Path::new(&staging_path).exists() {
        let content = fs::read_to_string(&staging_path).unwrap();
        serde_json::from_str::<HashMap<String, String>>(&content).unwrap_or_default()
    } else {
        HashMap::new()
    };

    let target_path = Path::new(path);
    if target_path.is_file() {
        let content = fs::read_to_string(target_path).unwrap();
        staged_files.insert(path.to_string(), content);
    } else if target_path.is_dir() {
        add_directory(target_path, &mut staged_files);
    } else {
        println!("Invalid path: '{}'", path);
        return;
    }

    fs::write(
        &staging_path,
        serde_json::to_string_pretty(&staged_files).unwrap(),
    )
    .unwrap();
    println!("Added '{}' to staging.", path);
}

fn add_directory(dir: &Path, staged_files: &mut HashMap<String, String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            add_directory(&path, staged_files); // Recurse into subdirectories
        } else if path.is_file() {
            let relative_path = path.to_str().unwrap().to_string();
            if !relative_path.starts_with(".rustvcs") {
                let content = fs::read_to_string(&path).unwrap();
                staged_files.insert(relative_path, content);
            }
        }
    }
}

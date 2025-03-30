use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn add_file(filename: &str) {
    let vcs_dir = ".rustvcs";
    let index_path = format!("{}/index.json", vcs_dir);

    if !Path::new(vcs_dir).exists() {
        println!("Repository not initialized. Run `cargo run init`.");
        return;
    }

    let file_content = fs::read_to_string(filename).expect("Failed to read file");
    let mut index: HashMap<String, String> =
        serde_json::from_str(&fs::read_to_string(&index_path).unwrap_or("{}".to_string())).unwrap();

    index.insert(filename.to_string(), file_content);
    fs::write(index_path, serde_json::to_string_pretty(&index).unwrap()).unwrap();
    println!("Added {} to staging.", filename);
}

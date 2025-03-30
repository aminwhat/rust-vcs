use std::fs;
use std::path::Path;

pub fn init_repo() {
    let vcs_dir = ".rustvcs";
    if Path::new(vcs_dir).exists() {
        println!("Repository already initialized.");
        return;
    }
    fs::create_dir(vcs_dir).unwrap();
    fs::create_dir(format!("{}/commits", vcs_dir)).unwrap();
    fs::write(format!("{}/index.json", vcs_dir), "{}").unwrap();
    println!("Initialized empty RustVCS repository.");
}

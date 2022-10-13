use std::{process, path::{PathBuf}};

fn main() {
    let frontend_path = PathBuf::from("src/frontend");
    
    process::Command::new("npm")
        .current_dir(frontend_path.clone())
        .arg("install")
        .status()
    .expect("npm install failed to execute");

    process::Command::new("npm")
        .current_dir(frontend_path)
        .args(["run", "build"])
        .status()
    .expect("Failed to build frontend app");

    println!("cargo:rerun-if-changed=src/frontend")
}
use std::{process, path::{PathBuf}};

fn main() {
    let frontend_path = PathBuf::from("src/frontend");
    
    let install = process::Command::new("npm")
            .current_dir(frontend_path.clone())
            .arg("install")
            .spawn()
            .expect("install failed")
        .wait()
    .expect("error while waiting for npm install");

    if !install.success() {panic!("Failed to install")}

    let build = process::Command::new("npm")
                .current_dir(frontend_path.clone())
                .args(["run", "build"])
                .spawn()
        .expect("build failed")
        .wait()
    .expect("error while waiting for npm build");

    if !build.success() {panic!("Failed to build")}

    println!("cargo:rerun-if-changed=src/frontend")
}
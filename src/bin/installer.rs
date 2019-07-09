use std::env;
use std::path::PathBuf;
use std::process::Command;
use fs_extra::dir;

type PathParts = [&'static str];

fn path_from_cwd(parts: &PathParts) -> PathBuf {
    let mut pathbuf = env::current_dir().unwrap();
    for part in parts.iter() {
        pathbuf.push(part);
    }
    pathbuf
}

fn copy_resources_dir(dest_dir: &PathParts) {
    let resources_dir = ["pathfinder", "resources"];
    let copy_options = dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: true,
        depth: 0
    };
    let dest_pathbuf = path_from_cwd(dest_dir);
    let erase = false;

    println!("{} -> {}", resources_dir.join("/"), dest_dir.join("/"));
    dir::create_all(dest_pathbuf.clone(), erase).unwrap();
    dir::copy(path_from_cwd(&resources_dir), dest_pathbuf, &copy_options).unwrap();
}

fn build() {
    let mut child = Command::new("cargo")
        .arg("build")
        .arg("--lib")
        .spawn()
        .expect("failed to run cargo");
    let ecode = child.wait().expect("failed to wait on cargo");
    if !ecode.success() {
        std::process::exit(1);
    }
}

fn main() {
    println!("Building Pathfinder Unity plugin...");
    build();
    println!("Copying resource directories...");
    copy_resources_dir(&["unity-project", "Assets", "StreamingAssets", "pathfinder"]);
    copy_resources_dir(&["dist", "unity-project_Data", "StreamingAssets", "pathfinder"]);
}

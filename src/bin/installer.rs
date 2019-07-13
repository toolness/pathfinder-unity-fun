use std::env;
use std::path::PathBuf;
use std::process::Command;
use fs_extra::{dir, file};

type PathParts = [&'static str];

#[cfg(debug_assertions)]
const TARGET_DIR: &'static str = "debug";

#[cfg(not(debug_assertions))]
const TARGET_DIR: &'static str = "release";

const RESOURCES_DIR: [&'static str; 2] = ["pathfinder", "resources"];
const DLL_PATH: [&'static str; 3] = ["target", TARGET_DIR, "pathfinder_c_api_fun.dll"];

fn path_from_cwd(parts: &PathParts) -> PathBuf {
    let mut pathbuf = env::current_dir().unwrap();
    for part in parts.iter() {
        pathbuf.push(part);
    }
    pathbuf
}

fn copy_resources_dir(dest_dir: &PathParts) {
    let copy_options = dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
        copy_inside: true,
        depth: 0
    };
    let dest_pathbuf = path_from_cwd(dest_dir);
    let erase = false;

    println!("{} -> {}", RESOURCES_DIR.join("/"), dest_dir.join("/"));
    dir::create_all(dest_pathbuf.clone(), erase).unwrap();
    dir::copy(path_from_cwd(&RESOURCES_DIR), dest_pathbuf, &copy_options).unwrap();
}

fn copy_dll(dest_dir: &PathParts) {
    let mut dest = Vec::from(dest_dir);
    dest.push("GfxPluginPathfinder.dll");
    let options = file::CopyOptions {
        buffer_size: 64000,
        overwrite: true,
        skip_exist: false,
    };

    println!("{} -> {}", DLL_PATH.join("/"), dest.join("/"));
    dir::create_all(path_from_cwd(dest_dir), false).unwrap();
    file::copy(path_from_cwd(&DLL_PATH), path_from_cwd(&dest), &options).unwrap();
}

fn build() {
    let mut cmd = Command::new("cargo");

    cmd.arg("build").arg("--lib");

    if !cfg!(debug_assertions) {
        cmd.arg("--release");
    }

    let mut child = cmd
        .spawn()
        .expect("failed to run cargo");
    let ecode = child.wait().expect("failed to wait on cargo");
    if !ecode.success() {
        std::process::exit(1);
    }
}

fn main() {
    if cfg!(debug_assertions) {
        println!("Building Pathfinder Unity plugin for debugging...");
    } else {
        println!("Building Pathfinder Unity plugin with optimizations...");
    }

    build();

    println!("Copying plugin to Unity projects...");
    copy_dll(&["unity-project", "Assets"]);
    copy_dll(&["dist", "unity-project_Data", "Plugins"]);

    println!("Copying resource directory to Unity projects...");
    copy_resources_dir(&["unity-project", "Assets", "StreamingAssets", "pathfinder"]);
    copy_resources_dir(&["dist", "unity-project_Data", "StreamingAssets", "pathfinder"]);
}

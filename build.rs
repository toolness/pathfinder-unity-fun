use std::env;
use std::fs;
use std::path::PathBuf;

use csharpbindgen::ignores::Ignores;
use csharpbindgen::create_csharp_bindings;

const PATHFINDER_UNITY_API_RS: [&'static str; 2] = ["src", "pathfinder_unity_api.rs"];

type PathParts = [&'static str];

fn read_file(path_parts: &PathParts) -> String {
    let path = path_from_cwd(path_parts);

    if !path.exists() {
        panic!("Expected file to exist: {}", path.to_string_lossy());
    }

    if let Ok(code) = fs::read_to_string(&path) {
        code
    } else {
        panic!("Unable to read {}!", path.to_string_lossy())
    }
}

fn path_from_cwd(parts: &PathParts) -> PathBuf {
    let mut pathbuf = env::current_dir().unwrap();
    for part in parts.iter() {
        pathbuf.push(part);
    }
    pathbuf
}

fn has_content_changed(path: &PathBuf, new_content: &String) -> bool {
    if path.exists() {
        let curr_content = fs::read_to_string(path.clone()).unwrap();
        if curr_content == *new_content {
            return false;
        }
    }
    true
}

fn write_if_changed(path_parts: &PathParts, content: &String) {
    let path = path_from_cwd(&path_parts);

    if has_content_changed(&path, &content) {
        println!("Writing {}.", path_parts.join("/"));

        fs::write(path, content).unwrap();
    }
}

fn build_pathfinder_rust_code() {
    let mut content = read_file(&["pathfinder", "c", "src", "lib.rs"])
        .replace("extern \"C\"", "extern \"stdcall\"");

    content = String::from(
        "// This file has been auto-generated, please do not edit it.\n\n"
    ) + &content;

    write_if_changed(&PATHFINDER_UNITY_API_RS, &content);
}

fn build_pathfinder_csharp_code() {
    let code = read_file(&PATHFINDER_UNITY_API_RS);
    let ignores = Ignores::from_static_array(&[
        "PFGLFunctionLoader",
        "PFCanvasFontContextCreateWithFonts",
        "PFCanvasCreateScene",
        "PFRendererOptions",
        "PFScene*",
        "PFGL*",
        "PFMetal*"
    ]);
    let bindings_code = create_csharp_bindings("PF", "GfxPluginPathfinder", &code, &ignores);

    write_if_changed(&["unity-project", "Assets", "PF.cs"], &bindings_code);
}

pub fn main() {
    build_pathfinder_rust_code();
    build_pathfinder_csharp_code();
}

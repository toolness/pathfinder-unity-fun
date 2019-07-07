use std::env;
use std::fs;
use std::path::PathBuf;
use csharpbindgen::CSAccess;

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
        // Unity uses the "stdcall" calling convention, so we'll substitute
        // the C API's default "C" calling convention for it.
        //
        // TODO: It seems possible to specify the calling convention on the
        // C# side via an attribute, so we could potentially just use that
        // instead of this hackery.
        .replace("extern \"C\"", "extern \"stdcall\"")
        // This is an unused import for Windows OpenGL and we don't want
        // an annoying warning logged so we'll just remove it.
        // 
        // TODO: Change Pathfinder upstream to not import it if the
        // build configuration doesn't need it, which will obviate the
        // need for this line.
        .replace("use foreign_types::ForeignTypeRef;", "");

    content = String::from(
        "// This file has been auto-generated, please do not edit it.\n\n"
    ) + &content;

    write_if_changed(&PATHFINDER_UNITY_API_RS, &content);
}

fn build_pathfinder_csharp_code() {
    let srcfile = PATHFINDER_UNITY_API_RS;
    let code = read_file(&srcfile);
    let bindings_result = csharpbindgen::Builder::new("GfxPluginPathfinder", code)
        .class_name("PF")
        .ignore(&[
            "PFGLFunctionLoader",
            "PFCanvasFontContextCreateWithFonts",
            "PFCanvasCreateScene",
            "PFRendererOptions",
            "PFScene*",
            "PFGL*",
            "PFMetal*"
        ])
        .access("PFTextMetrics", CSAccess::Public)
        .generate();

    match bindings_result {
        Err(err) => {
            println!(
                "Unable to generate Pathfinder C# code from {}.\n{}.",
                srcfile.join("/"),
                err
            );
            std::process::exit(1);
        },
        Ok(bindings_code) => {
            write_if_changed(&["unity-project", "Assets", "Pathfinder", "PF.cs"], &bindings_code);
        }
    }
}

pub fn main() {
    build_pathfinder_rust_code();
    build_pathfinder_csharp_code();
}

use std::env;
use std::fs;

use csharpbindgen::ignores::Ignores;
use csharpbindgen::create_csharp_bindings;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("..");
    path.push("src");
    path.push("pathfinder_unity_api.rs");

    if !path.exists() {
        panic!("Expected file to exist: {}", path.to_string_lossy());
    }

    let code = fs::read_to_string(path).expect("unable to read rust source file");
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

    println!("{}", bindings_code);
}

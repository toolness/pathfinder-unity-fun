use std::env;
use std::fs;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("..");
    path.push("src");
    path.push("pathfinder_unity_api.rs");

    if !path.exists() {
        panic!("Expected file to exist: {}", path.to_string_lossy());
    }

    let code = fs::read_to_string(path).expect("unable to read rust source file");

    let bindings_code = csharpbindgen::create_csharp_bindings(&code, &[
        "PFGLFunctionLoader"
    ]);

    println!("{}", bindings_code);
}

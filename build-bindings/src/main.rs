use std::env;
use std::fs;
use std::path::PathBuf;


fn path_from_cwd(parts: &[&'static str]) -> PathBuf {
    let mut pathbuf = env::current_dir().unwrap();
    for part in parts.iter() {
        pathbuf.push(part);
    }
    pathbuf
}

pub fn main() {
    let c_api_path = path_from_cwd(&["..", "pathfinder", "c", "src", "lib.rs"]);

    if !c_api_path.exists() {
        panic!("Expected {} to exist!", c_api_path.to_string_lossy());
    }

    let content = fs::read_to_string(c_api_path)
        .unwrap()
        .replace("extern \"C\"", "extern \"stdcall\"");

    let plugin_parts = ["..", "src", "pathfinder_unity_api.rs"];
    let plugin_api_path = path_from_cwd(&plugin_parts);

    println!("Writing {}.", plugin_parts.join("/"));

    fs::write(plugin_api_path, content).unwrap();
}

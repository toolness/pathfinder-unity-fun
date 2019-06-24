use std::env;
use std::fs;

pub fn main() {
    let mut c_api_path = env::current_dir().unwrap();
    for part in ["..", "pathfinder", "c", "src", "lib.rs"].iter() {
        c_api_path.push(part);
    }

    if !c_api_path.exists() {
        panic!("Expected {} to exist!", c_api_path.to_string_lossy());
    }

    let content = fs::read_to_string(c_api_path)
        .unwrap()
        .replace("extern \"C\"", "extern \"stdcall\"");

    let mut plugin_api_path = env::current_dir().unwrap();
    for part in ["..", "src", "pathfinder_unity_api.rs"].iter() {
        plugin_api_path.push(part);
    }

    println!("Writing {}.", plugin_api_path.to_string_lossy());

    fs::write(plugin_api_path, content).unwrap();
}

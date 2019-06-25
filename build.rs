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

fn has_content_changed(path: &PathBuf, new_content: &String) -> bool {
    if path.exists() {
        let curr_content = fs::read_to_string(path.clone()).unwrap();
        if curr_content == *new_content {
            return false;
        }
    }
    true
}

pub fn main() {
    let c_api_path = path_from_cwd(&["pathfinder", "c", "src", "lib.rs"]);

    if !c_api_path.exists() {
        panic!("Expected {} to exist!", c_api_path.to_string_lossy());
    }

    let mut content = fs::read_to_string(c_api_path)
        .unwrap()
        .replace("extern \"C\"", "extern \"stdcall\"");

    content = String::from(
        "// This file has been auto-generated, please do not edit it.\n\n"
    ) + &content;

    let plugin_parts = ["src", "pathfinder_unity_api.rs"];
    let plugin_api_path = path_from_cwd(&plugin_parts);

    if has_content_changed(&plugin_api_path, &content) {
        println!("Writing {}.", plugin_parts.join("/"));

        fs::write(plugin_api_path, content).unwrap();
    }
}

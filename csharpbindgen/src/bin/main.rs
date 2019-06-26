use std::env;
use std::fs;
use syn::Item;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("..");
    path.push("src");
    path.push("pathfinder_unity_api.rs");

    if !path.exists() {
        panic!("Expected file to exist: {}", path.to_string_lossy());
    }

    let code = fs::read_to_string(path).expect("unable to read rust source file");

    let syntax = syn::parse_file(&code).expect("unable to parse rust source file");

    // println!("{:#?}", syntax);

    for item in syntax.items.iter() {
        match item {
            Item::Struct(item_struct) => {
                println!("// TODO: Define struct {}", item_struct.ident.to_string());
            },
            Item::Fn(item_fn) => {
                if item_fn.abi.is_some() {
                    println!("// TODO: Define fn {}()", item_fn.ident.to_string());
                }
            },
            _ => {}
        }
    }
}

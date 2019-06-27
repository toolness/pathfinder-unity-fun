use syn::Item;

pub fn create_csharp_bindings(rust_code: &String) -> String {
    let syntax = syn::parse_file(&rust_code).expect("unable to parse rust source file");
    let mut lines = Vec::new();

    // println!("{:#?}", syntax);

    for item in syntax.items.iter() {
        match item {
            Item::Struct(item_struct) => {
                lines.push(format!("// TODO: Define struct {}", item_struct.ident.to_string()));
            },
            Item::Fn(item_fn) => {
                if item_fn.abi.is_some() {
                    lines.push(format!("// TODO: Define fn {}()", item_fn.ident.to_string()));
                }
            },
            _ => {}
        }
    }

    lines.join("\n")
}

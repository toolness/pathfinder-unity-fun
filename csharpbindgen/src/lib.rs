use syn::Item;

struct CSStruct {
    name: String,
}

impl CSStruct {
    pub fn from_rust_struct(rust_struct: &syn::ItemStruct) -> Self {
        CSStruct {
            name: rust_struct.ident.to_string()
        }
    }

    pub fn to_string(&self) -> String {
        format!("// TODO: Define struct {}", self.name)
    }
}

struct CSFunc {
    name: String
}

impl CSFunc {
    pub fn from_rust_struct(rust_fn: &syn::ItemFn) -> Self {
        CSFunc {
            name: rust_fn.ident.to_string()
        }
    }

    pub fn to_string(&self) -> String {
        format!("// TODO: Define fn {}()", self.name)
    }
}

struct CSFile {
    structs: Vec<CSStruct>,
    funcs: Vec<CSFunc>
}

impl CSFile {
    pub fn new() -> Self {
        CSFile {
            structs: vec![],
            funcs: vec![]
        }
    }

    pub fn from_rust_file(rust_file: &syn::File) -> Self {
        let mut program = Self::new();

        for item in rust_file.items.iter() {
            match item {
                Item::Struct(item_struct) => {
                    program.structs.push(CSStruct::from_rust_struct(&item_struct));
                },
                Item::Fn(item_fn) => {
                    if item_fn.abi.is_some() {
                        program.funcs.push(CSFunc::from_rust_struct(&item_fn));
                    }
                },
                _ => {}
            }
        }

        program
    }

    pub fn to_string(&self) -> String {
        let mut lines = Vec::new();

        for st in self.structs.iter() {
            lines.push(st.to_string());
        }
        for func in self.funcs.iter() {
            lines.push(func.to_string());
        }

        lines.join("\n")
    }
}

pub fn create_csharp_bindings(rust_code: &String) -> String {
    let syntax = syn::parse_file(&rust_code).expect("unable to parse rust source file");
    let program = CSFile::from_rust_file(&syntax);

    program.to_string()
}

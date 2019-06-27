use syn::Item;

struct CSStructField {
    name: String,
}

impl CSStructField {
    pub fn from_named_rust_field(rust_field: &syn::Field) -> Self {
        CSStructField {
            name: rust_field.ident.as_ref().unwrap().to_string()
        }
    }
}

struct CSStruct {
    name: String,
    fields: Vec<CSStructField>
}

impl CSStruct {
    pub fn from_rust_struct(rust_struct: &syn::ItemStruct) -> Self {
        let mut fields = vec![];

        if let syn::Fields::Named(rust_fields) = &rust_struct.fields {
            for rust_field in rust_fields.named.iter() {
                fields.push(CSStructField::from_named_rust_field(rust_field));
            }
        }
        CSStruct {
            name: rust_struct.ident.to_string(),
            fields
        }
    }

    pub fn to_string(&self) -> String {
        let fields: Vec<String> = self.fields.iter().map(|f| f.name.clone()).collect();
        format!("// TODO: Define struct {} w/ fields {}", self.name, fields.join(", "))
    }
}

struct CSFuncArg {
    name: String,
}

impl CSFuncArg {
    pub fn from_rust_arg_captured(rust_arg: &syn::ArgCaptured) -> Self {
        CSFuncArg {
            name: String::from("unknown") // TODO: Get actual arg name.
        }
    }
}

struct CSFunc {
    name: String,
    args: Vec<CSFuncArg>
}

impl CSFunc {
    pub fn from_rust_struct(rust_fn: &syn::ItemFn) -> Self {
        let args = vec![];

        // TODO: Populate args.

        CSFunc {
            name: rust_fn.ident.to_string(),
            args
        }
    }

    pub fn to_string(&self) -> String {
        // TODO: List args.
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

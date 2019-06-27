use syn::Item;

struct CSStruct {
    name: String,
}

impl CSStruct {
    pub fn to_string(&self) -> String {
        format!("// TODO: Define struct {}", self.name)
    }
}

struct CSFunc {
    name: String
}

impl CSFunc {
    pub fn to_string(&self) -> String {
        format!("// TODO: Define fn {}()", self.name)
    }
}

struct CSProgram {
    structs: Vec<CSStruct>,
    funcs: Vec<CSFunc>
}

impl CSProgram {
    pub fn new() -> Self {
        CSProgram {
            structs: vec![],
            funcs: vec![]
        }
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

fn into_cs_program(syntax: syn::File) -> CSProgram {
    let mut program = CSProgram::new();

    for item in syntax.items.iter() {
        match item {
            Item::Struct(item_struct) => {
                program.structs.push(CSStruct {
                    name: item_struct.ident.to_string()
                });
            },
            Item::Fn(item_fn) => {
                if item_fn.abi.is_some() {
                    program.funcs.push(CSFunc {
                        name: item_fn.ident.to_string()
                    });
                }
            },
            _ => {}
        }
    }

    program
}

pub fn create_csharp_bindings(rust_code: &String) -> String {
    let syntax = syn::parse_file(&rust_code).expect("unable to parse rust source file");
    let program = into_cs_program(syntax);

    program.to_string()
}

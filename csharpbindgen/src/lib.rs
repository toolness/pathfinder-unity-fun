use syn::Item;

pub mod ignores;

use ignores::Ignores;

struct CSTypeDef {
    name: String,
    ty: CSType
}

impl CSTypeDef {
    pub fn from_rust_type_def(rust_type_def: &syn::ItemType) -> Self {
        CSTypeDef {
            name: rust_type_def.ident.to_string(),
            ty: CSType::from_rust_type(&rust_type_def.ty)
        }
    }
}

struct CSType {
    name: String,
    is_ptr: bool
}

impl CSType {
    pub fn from_rust_type(rust_type: &syn::Type) -> Self {
        match rust_type {
            syn::Type::Path(type_path) => {
                let last = type_path.path.segments.last()
                  .expect("expected at least one path segment on type!");
                CSType {
                    name: last.value().ident.to_string(),
                    is_ptr: false
                }
            },
            syn::Type::Ptr(type_ptr) => {
                let mut wrapped_type = CSType::from_rust_type(&type_ptr.elem);
                wrapped_type.is_ptr = true;
                wrapped_type
            },
            _ => { panic!("Unsupported type: {:?}", rust_type) }
        }
    }

    pub fn to_string(&self) -> String {
        if self.is_ptr {
            format!("*{}", self.name)
        } else {
            self.name.clone()
        }
    }
}

struct CSStructField {
    name: String,
    ty: CSType,
}

impl CSStructField {
    pub fn from_named_rust_field(rust_field: &syn::Field) -> Self {
        CSStructField {
            name: rust_field.ident.as_ref().unwrap().to_string(),
            ty: CSType::from_rust_type(&rust_field.ty)
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
        let fields: Vec<String> = self.fields
          .iter()
          .map(|f| format!("{}: {}", f.name, f.ty.to_string()))
          .collect();
        format!("// TODO: Define struct {} {{ {} }}", self.name, fields.join(", "))
    }
}

struct CSFuncArg {
    name: String,
    ty: CSType
}

impl CSFuncArg {
    pub fn from_rust_arg_captured(rust_arg: &syn::ArgCaptured) -> Self {
        if let syn::Pat::Ident(pat_ident) = &rust_arg.pat {
            let name = pat_ident.ident.to_string();
            CSFuncArg {
                name,
                ty: CSType::from_rust_type(&rust_arg.ty)
            }
        } else {
            panic!("Unexpected captured arg pattern {:?}", rust_arg.pat);
        }
    }
}

struct CSFunc {
    name: String,
    args: Vec<CSFuncArg>
}

impl CSFunc {
    pub fn from_rust_fn(rust_fn: &syn::ItemFn) -> Self {
        let mut args = vec![];

        for input in rust_fn.decl.inputs.iter() {
            if let syn::FnArg::Captured(cap) = input {
                args.push(CSFuncArg::from_rust_arg_captured(&cap));
            } else {
                panic!("Unexpected input {:?}", input);
            }
        }

        CSFunc {
            name: rust_fn.ident.to_string(),
            args
        }
    }

    pub fn to_string(&self) -> String {
        let arg_names: Vec<String> = self.args
          .iter()
          .map(|f| format!("{}: {}", f.name, f.ty.to_string()))
          .collect();
        format!("// TODO: Define fn {}({})", self.name, arg_names.join(", "))
    }
}

struct CSFile {
    structs: Vec<CSStruct>,
    funcs: Vec<CSFunc>,
    type_defs: Vec<CSTypeDef>
}

impl CSFile {
    pub fn new() -> Self {
        CSFile {
            structs: vec![],
            funcs: vec![],
            type_defs: vec![]
        }
    }

    pub fn from_rust_file(rust_file: &syn::File, ignores: &Ignores) -> Self {
        let mut program = Self::new();

        for item in rust_file.items.iter() {
            match item {
                Item::Struct(item_struct) => {
                    program.structs.push(CSStruct::from_rust_struct(&item_struct));
                },
                Item::Fn(item_fn) => {
                    if item_fn.abi.is_some() {
                        program.funcs.push(CSFunc::from_rust_fn(&item_fn));
                    }
                },
                Item::Type(item_type) => {
                    if !ignores.ignore(&item_type.ident) {
                        program.type_defs.push(CSTypeDef::from_rust_type_def(&item_type));
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

pub fn create_csharp_bindings(rust_code: &String, ignores: &Ignores) -> String {
    let syntax = syn::parse_file(&rust_code).expect("unable to parse rust source file");
    let program = CSFile::from_rust_file(&syntax, ignores);

    program.to_string()
}

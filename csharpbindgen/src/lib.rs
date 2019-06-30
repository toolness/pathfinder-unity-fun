use std::collections::HashMap;
use syn::Item;
use std::fmt::{Formatter, Display};
use std::fmt;
use std::rc::Rc;

pub mod ignores;

use ignores::Ignores;

const INDENT: &'static str = "    ";

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

#[derive(Clone)]
struct CSType {
    name: String,
    is_ptr: bool,
    st: Option<Rc<CSStruct>>
}

impl CSType {
    pub fn from_rust_type(rust_type: &syn::Type) -> Self {
        match rust_type {
            syn::Type::Path(type_path) => {
                let last = type_path.path.segments.last()
                  .expect("expected at least one path segment on type!");
                CSType {
                    name: last.value().ident.to_string(),
                    is_ptr: false,
                    st: None
                }
            },
            syn::Type::Ptr(type_ptr) => {
                let mut wrapped_type = CSType::from_rust_type(&type_ptr.elem);
                assert_ne!(
                    wrapped_type.is_ptr, true,
                    "Double pointers for {} are unsupported!", wrapped_type.name
                );
                wrapped_type.is_ptr = true;
                wrapped_type
            },
            _ => { panic!("Unsupported type: {:?}", rust_type) }
        }
    }
}

impl Display for CSType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = to_cs_primitive(&self.name);
        if self.is_ptr {
            if self.st.is_some() {
                write!(f, "ref {}", name)
            } else {
                write!(f, "IntPtr /* {} */", name)
            }
        } else {
            write!(f, "{}", name)
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

    pub fn to_string(&self) -> String {
        to_cs_var_decl(&self.ty, &self.name)
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
}

impl Display for CSStruct {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "[Serializable]")?;
        writeln!(f, "[StructLayout(LayoutKind.Sequential)]")?;
        writeln!(f, "struct {} {{", self.name)?;
        for field in self.fields.iter() {
            writeln!(f, "{}public {};", INDENT, field.to_string())?;
        }

        let constructor_args: Vec<String> = self.fields
          .iter()
          .map(|field| field.to_string())
          .collect();
        writeln!(f, "\n{}public {}({}) {{", INDENT, self.name, constructor_args.join(", "))?;
        for field in self.fields.iter() {
            let name = munge_cs_name(&field.name);
            writeln!(f, "{}{}this.{} = {};", INDENT, INDENT, name, name)?;
        }
        writeln!(f, "{}}}", INDENT)?;

        writeln!(f, "}}")
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

    pub fn to_string(&self) -> String {
        to_cs_var_decl(&self.ty, &self.name)
    }
}

struct CSFunc {
    name: String,
    args: Vec<CSFuncArg>,
    return_ty: Option<CSType>
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

        let return_ty = match &rust_fn.decl.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => {
                Some(CSType::from_rust_type(&ty))
            }
        };

        CSFunc {
            name: rust_fn.ident.to_string(),
            args,
            return_ty
        }
    }
}

impl Display for CSFunc {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let return_ty = match &self.return_ty {
            None => String::from("void"),
            Some(ty) => ty.to_string()
        };
        let args: Vec<String> = self.args
          .iter()
          .map(|arg| arg.to_string())
          .collect();
        write!(f, "public static extern {} {}({});", return_ty, self.name, args.join(", "))
    }
}

struct CSFile {
    class_name: String,
    dll_name: String,
    structs: Vec<Rc<CSStruct>>,
    funcs: Vec<CSFunc>,
    type_defs: HashMap<String, CSTypeDef>
}

impl CSFile {
    pub fn new<T: AsRef<str>>(class_name: T, dll_name: T) -> Self {
        CSFile {
            class_name: String::from(class_name.as_ref()),
            dll_name: String::from(dll_name.as_ref()),
            structs: vec![],
            funcs: vec![],
            type_defs: HashMap::new()
        }
    }

    pub fn populate_from_rust_file(&mut self, rust_file: &syn::File, ignores: &Ignores) {
        for item in rust_file.items.iter() {
            match item {
                Item::Struct(item_struct) => {
                    if !ignores.ignore(&item_struct.ident) {
                        let s = Rc::new(CSStruct::from_rust_struct(&item_struct));
                        self.structs.push(s);
                    }
                },
                Item::Fn(item_fn) => {
                    if item_fn.abi.is_some() {
                        if !ignores.ignore(&item_fn.ident) {
                            self.funcs.push(CSFunc::from_rust_fn(&item_fn));
                        }
                    }
                },
                Item::Type(item_type) => {
                    if !ignores.ignore(&item_type.ident) {
                        let type_def = CSTypeDef::from_rust_type_def(&item_type);
                        self.type_defs.insert(type_def.name.clone(), type_def);
                    }
                },
                _ => {}
            }
        }
    }

    fn resolve_types(&mut self) {
        let mut struct_map: HashMap<&str, &Rc<CSStruct>> = HashMap::new();

        for st in self.structs.iter() {
            struct_map.insert(&st.name, &st);
        }

        for func in self.funcs.iter_mut() {
            for arg in func.args.iter_mut() {
                if let Some(ty) = resolve_type_def(&arg.ty, &self.type_defs) {
                    arg.ty = ty;
                }
                if let Some(st) = struct_map.get(&arg.ty.name.as_ref()) {
                    arg.ty.st = Some((*st).clone());
                }
            }
            if let Some(return_ty) = &func.return_ty {
                if let Some(ty) = resolve_type_def(return_ty, &self.type_defs) {
                    func.return_ty = Some(ty);
                }
            }
        }
    }
}

impl Display for CSFile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "// This file has been auto-generated, please do not edit it.\n")?;
        writeln!(f, "using System;")?;
        writeln!(f, "using System.Runtime.InteropServices;\n")?;

        for st in self.structs.iter() {
            writeln!(f, "{}", st)?;
        }
        writeln!(f, "class {} {{", self.class_name)?;
        for func in self.funcs.iter() {
            writeln!(f, "{}[DllImport(\"{}\")]", INDENT, self.dll_name)?;
            writeln!(f, "{}{}\n", INDENT, func)?;
        }
        writeln!(f, "}}")
    }
}

pub fn create_csharp_bindings<T: AsRef<str>>(
    class_name: T,
    dll_name: T,
    rust_code: &String,
    ignores: &Ignores
) -> String {
    let syntax = syn::parse_file(&rust_code).expect("unable to parse rust source file");
    let mut program = CSFile::new(class_name, dll_name);
    program.populate_from_rust_file(&syntax, ignores);
    program.resolve_types();
    format!("{}", program)
}

fn resolve_type_def(ty: &CSType, type_defs: &HashMap<String, CSTypeDef>) -> Option<CSType> {
    if let Some(type_def) = type_defs.get(&ty.name) {
        assert!(
            !(ty.is_ptr && type_def.ty.is_ptr),
            "Double pointer to {} via type {} is unsupported!",
            type_def.ty.name,
            type_def.name
        );
        Some(type_def.ty.clone())
    } else {
        None
    }
}

fn munge_cs_name<'a>(name: &'a str) -> &'a str {
    match name {
        "string" => "str",
        _ => name
    }
}

fn to_cs_primitive<'a>(type_name: &'a str) -> &'a str {
    match type_name {
        "u8" => "byte",
        "f32" => "float",
        "i32" => "Int32",
        "u32" => "UInt32",
        "usize" => "UIntPtr",
        _ => type_name
    }
}

fn to_cs_var_decl<T: AsRef<str>>(ty: &CSType, name: T) -> String {
    format!("{} {}", ty, munge_cs_name(name.as_ref()))
}

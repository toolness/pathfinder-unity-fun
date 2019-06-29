use std::collections::BTreeSet;

pub struct Ignores {
    exact: BTreeSet<String>,
    prefixes: Vec<String>
}

impl Ignores {
    pub fn new() -> Self {
        Ignores {
            exact: BTreeSet::new(),
            prefixes: vec![],
        }
    }

    pub fn from_static_array(ignore: &[&str]) -> Self {
        let mut result = Ignores::new();

        for name in ignore.iter() {
            if (*name).ends_with("*") {
                let substr = &(*name)[..(*name).len()-1];
                result.prefixes.push(String::from(substr));
            }
            result.exact.insert(String::from(*name));
        }

        result
    }

    pub fn ignore(&self, ident: &syn::Ident) -> bool {
        let s = ident.to_string();

        if self.exact.contains(&s) {
            return true;
        }

        for prefix in self.prefixes.iter() {
            if s.starts_with(prefix) {
                return true;
            }
        }

        return false;
    }
}

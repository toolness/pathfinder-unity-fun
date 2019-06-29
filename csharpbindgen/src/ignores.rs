use std::collections::BTreeSet;

pub struct Ignores {
    exact: BTreeSet<String>
}

impl Ignores {
    pub fn new() -> Self {
        Ignores {
            exact: BTreeSet::new(),
        }
    }

    pub fn from_static_array(ignore: &[&str]) -> Self {
        let mut result = Ignores::new();

        for name in ignore.iter() {
            result.exact.insert(String::from(*name));
        }

        result
    }

    pub fn ignore(&self, ident: &syn::Ident) -> bool {
        self.exact.contains(&ident.to_string())
    }
}

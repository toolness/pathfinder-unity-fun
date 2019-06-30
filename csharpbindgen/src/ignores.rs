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

    pub fn ignore_str<T: AsRef<str>>(&self, value: T) -> bool {
        let s = value.as_ref();

        if self.exact.contains(s) {
            return true;
        }

        for prefix in self.prefixes.iter() {
            if s.starts_with(prefix) {
                return true;
            }
        }

        false
    }

    pub fn ignore(&self, ident: &syn::Ident) -> bool {
        self.ignore_str(&ident.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works_with_exact_matches() {
        let ig = Ignores::from_static_array(&["boop"]);
        assert_eq!(ig.ignore_str("boop"), true);
        assert_eq!(ig.ignore_str("boopy"), false);
    }

    #[test]
    fn test_it_works_with_prefixes() {
        let ig = Ignores::from_static_array(&["boop*"]);
        assert_eq!(ig.ignore_str("boop"), true);
        assert_eq!(ig.ignore_str("boopy"), true);
        assert_eq!(ig.ignore_str("funkyboop"), false);
    }
}

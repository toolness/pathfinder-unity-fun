use std::fmt;
use std::fmt::{Formatter, Display};

#[derive(Debug)]
pub enum Error {
    SynError(syn::Error),
    UnsupportedError(String)
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::SynError(err) => {
                write!(f, "Couldn't parse Rust code: {}", err)
            },
            Error::UnsupportedError(reason) => {
                write!(f, "Unable to export C# code because {}", reason)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SynError(ref err) => {
                Some(err)
            },
            Error::UnsupportedError(_) => {
                None
            }
        }
    }
}

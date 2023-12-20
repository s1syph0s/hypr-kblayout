use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum KbError {
    LayoutNotFound(String),
}

impl fmt::Display for KbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LayoutNotFound(s) => write!(f, "Layout {} not found!", s),
        }
    }
}

impl Error for KbError {
}

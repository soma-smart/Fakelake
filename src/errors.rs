use core::fmt;
use std::io;

#[derive(Debug)]
pub enum FakeLakeError {
    BadYAMLFormat(String),
    IOError(io::Error),
}

impl fmt::Display for FakeLakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // For now, use the debug derived version
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for FakeLakeError {
    fn from(error: io::Error) -> Self {
        FakeLakeError::IOError(error)
    }
}

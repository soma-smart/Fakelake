use core::fmt;
use std::io;

#[derive(Debug)]
pub enum FakeLakeError {
    BadYAMLFormat(String),
    IOError(io::Error),
    CSVError(csv::Error),
    JSONError(serde_json::Error),
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for FakeLakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // For now, use the debug derived version
        write!(f, "{:?}", self)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<io::Error> for FakeLakeError {
    fn from(error: io::Error) -> Self {
        FakeLakeError::IOError(error)
    }
}

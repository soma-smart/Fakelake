use core::fmt;
use std::io;

#[derive(Debug)]
pub enum FakeLakeError {
    BadYAMLFormat(String),
    IOError(io::Error),
    CSVError(csv::Error),
    JSONError(serde_json::Error),
    ParquetError(parquet::errors::ParquetError),
    ArrowError(arrow_schema::ArrowError),
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for FakeLakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FakeLakeError::BadYAMLFormat(msg) => write!(f, "{}", msg),
            FakeLakeError::IOError(err) => write!(f, "IO error: {}", err),
            FakeLakeError::CSVError(err) => write!(f, "CSV error: {}", err),
            FakeLakeError::JSONError(err) => write!(f, "JSON error: {}", err),
            FakeLakeError::ParquetError(err) => write!(f, "Parquet error: {}", err),
            FakeLakeError::ArrowError(err) => write!(f, "Arrow error: {}", err),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl From<io::Error> for FakeLakeError {
    fn from(error: io::Error) -> Self {
        FakeLakeError::IOError(error)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<parquet::errors::ParquetError> for FakeLakeError {
    fn from(error: parquet::errors::ParquetError) -> Self {
        FakeLakeError::ParquetError(error)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<arrow_schema::ArrowError> for FakeLakeError {
    fn from(error: arrow_schema::ArrowError) -> Self {
        FakeLakeError::ArrowError(error)
    }
}

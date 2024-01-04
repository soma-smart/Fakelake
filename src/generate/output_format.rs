use std::fmt;

use crate::config::Config;
use crate::errors::FakeLakeError;

pub trait OutputFormat {
    fn get_extension(&self) -> &str;
    fn generate_from_config(&self, config: &Config) -> Result<(), FakeLakeError>;
}
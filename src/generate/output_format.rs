use crate::config::Config;
use crate::errors::FakeLakeError;

pub trait OutputFormat {
    fn generate_from_config(config: &Config) -> Result<(), FakeLakeError>;
}
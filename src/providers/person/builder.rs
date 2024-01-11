use crate::errors::FakeLakeError;
use crate::providers::provider::Provider;

use super::email::EmailProvider;

use yaml_rust::Yaml;

pub fn get_corresponding_provider(mut provider_split: std::str::Split<'_, &str>, column: &Yaml) -> Result<Box<dyn Provider>, FakeLakeError> {
    match provider_split.next() {
        Some("email") => Ok(Box::new(EmailProvider::new_from_yaml(&column))),
        _ => Err(FakeLakeError::BadYAMLFormat("".to_string()))
    }
}
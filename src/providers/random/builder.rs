use crate::errors::FakeLakeError;
use crate::providers::provider::Provider;

use super::{ date, string };

use yaml_rust::Yaml;

pub fn get_corresponding_provider(mut provider_split: std::str::Split<'_, &str>, column: &Yaml) -> Result<Box<dyn Provider>, FakeLakeError> {
    match provider_split.next() {
        Some("Date") => date::builder::get_corresponding_provider(provider_split, &column),
        Some("String") => string::builder::get_corresponding_provider(provider_split, &column),
        _ => Err(FakeLakeError::BadYAMLFormat("".to_string()))
    }
}
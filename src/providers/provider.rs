use crate::errors::FakeLakeError;
use crate::providers;

use chrono::{NaiveDate, NaiveDateTime};
use core::fmt;
use yaml_rust::Yaml;

#[derive(PartialEq, fmt::Debug)]
pub enum Value {
    Bool(bool),
    Int32(i32),
    Float64(f64),
    String(String),
    Date(NaiveDate, String),
    Timestamp(NaiveDateTime, String),
}

pub trait CloneProvider {
    fn clone_box(&self) -> Box<dyn Provider>;
}

impl<T> CloneProvider for T
where
    T: 'static + Provider + Clone,
{
    fn clone_box(&self) -> Box<dyn Provider> {
        Box::new(self.clone())
    }
}

pub trait Provider: CloneProvider + Send + Sync {
    fn value(&self, index: u32) -> Value;
    fn new_from_yaml(column: &Yaml) -> Self
    where
        Self: Sized;
}

// Implement Debug for all types that implement Provider
#[cfg(not(tarpaulin_include))]
impl fmt::Debug for dyn Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Provider {{ }}")
    }
}

pub struct ProviderBuilder {}

impl ProviderBuilder {
    pub fn get_corresponding_provider(
        provider: &str,
        column: &Yaml,
    ) -> Result<Box<dyn Provider>, FakeLakeError> {
        let lowercased = provider.to_lowercase();
        let mut provider_split = lowercased.split('.');

        match provider_split.next() {
            Some("constant") => {
                providers::constant::builder::get_corresponding_provider(provider_split, column)
            }
            Some("increment") => {
                providers::increment::builder::get_corresponding_provider(provider_split, column)
            }
            Some("person") => {
                providers::person::builder::get_corresponding_provider(provider_split, column)
            }
            Some("random") => {
                providers::random::builder::get_corresponding_provider(provider_split, column)
            }
            _ => Err(unknown_provider(provider)),
        }
    }
}

pub fn unknown_provider(wrong_provider: &str) -> FakeLakeError {
    FakeLakeError::BadYAMLFormat(format!("Unknown provider: {}", wrong_provider))
}

#[cfg(test)]
mod tests {
    use super::ProviderBuilder;

    use yaml_rust::YamlLoader;

    #[test]
    fn given_valid_provider_should_return_provider() {
        let provider_names = [
            "constant.string",
            "increment.integer",
            "person.email",
            "random.string.alphanumeric",
        ];
        for provider_name in provider_names {
            let yaml_str = format!("name: name{}provider: {}", '\n', provider_name);
            let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

            match ProviderBuilder::get_corresponding_provider(provider_name, column) {
                Ok(_) => (),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn given_wrong_provider_should_return_error() {
        let provider_name = "not_a_provider";
        let yaml_str = format!("name: name{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        match ProviderBuilder::get_corresponding_provider(provider_name, column) {
            Err(_) => (),
            _ => panic!(),
        }
    }
}

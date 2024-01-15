use crate::errors::FakeLakeError;
use crate::providers;

use core::fmt;
use yaml_rust::Yaml;


#[derive(PartialEq, fmt::Debug)]
pub enum Value {
    Int32(i32),
    String(String),
    Date(i32),
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
    fn new_from_yaml(column: &Yaml) -> Self where Self: Sized;
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
    pub fn get_corresponding_provider(provider: &str, column: &Yaml) -> Result<Box<dyn Provider>, FakeLakeError> {
        let mut provider_split = provider.split(".");
        let provider_result: Result<Box<dyn Provider>, FakeLakeError> = match provider_split.next() {
            Some("Increment") => providers::increment::builder::get_corresponding_provider(provider_split, &column),
            Some("Person") => providers::person::builder::get_corresponding_provider(provider_split, &column),
            Some("Random") => providers::random::builder::get_corresponding_provider(provider_split, &column),
            _ => Err(unknown_provider(provider))
        };

        match provider_result {
            Ok(_) => provider_result,
            Err(_) => Err(FakeLakeError::BadYAMLFormat(format!("Unknown provider: {}", provider)))
        }
    }
}

pub fn unknown_provider(wrong_provider: &str) -> FakeLakeError {
    FakeLakeError::BadYAMLFormat(format!("Unknown provider: {}", wrong_provider))
}
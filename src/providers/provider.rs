use core::fmt;

use crate::errors::FakeLakeError;
use crate::providers::integer::auto_increment::AutoIncrementProvider;
use crate::providers::string::{ email::EmailProvider, string::StringProvider };
use crate::providers::date::date::DateProvider;

use arrow_schema::DataType;
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
    fn get_parquet_type(&self) -> DataType;
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
        match provider {
            "auto-increment" => Ok(Box::new(AutoIncrementProvider::new_from_yaml(&column))),
            "email" => Ok(Box::new(EmailProvider::new_from_yaml(&column))),
            "date" => Ok(Box::new(DateProvider::new_from_yaml(&column))),
            "string" => Ok(Box::new(StringProvider::new_from_yaml(&column))),
            _ => Err(FakeLakeError::BadYAMLFormat("Unknown provider: {{provider}}".to_string())),
        }
    }
}
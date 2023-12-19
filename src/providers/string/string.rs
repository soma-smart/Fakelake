use arrow_schema::DataType;
use yaml_rust::Yaml;

use crate::providers::provider::{Provider, Value};
use super::utils::random_characters;

pub struct StringProvider;

impl Provider for StringProvider {
    fn value(&self, _: u32) -> Value {
        Value::String(random_characters(10))
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Utf8;
    }
    fn new_from_yaml(_: &Yaml) -> StringProvider {
        return StringProvider;
    }
}

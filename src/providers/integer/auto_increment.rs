use arrow_schema::DataType;
use yaml_rust::Yaml;

use crate::providers::provider::{Provider, Value};

pub struct AutoIncrementProvider {
    pub start: Option<u32>,
}

impl Provider for AutoIncrementProvider {
    fn value(&self, index: u32) -> Value {
        return Value::Int32(index as i32);
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Int32;
    }
    fn new_from_yaml(yaml: &Vec<Yaml>) -> AutoIncrementProvider {
        return AutoIncrementProvider { start:None };
    }
}

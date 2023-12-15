use arrow_schema::DataType;
use yaml_rust::Yaml;

use crate::providers::provider::{Provider, Value};

const DEFAULT_START: i64 = 0;

pub struct AutoIncrementProvider {
    pub start: i32,
}

impl Provider for AutoIncrementProvider {
    fn value(&self, index: u32) -> Value {
        return Value::Int32(self.start + (index as i32));
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Int32;
    }
    fn new_from_yaml(column: &Yaml) -> AutoIncrementProvider {
        let start_option = column["start"].as_i64().unwrap_or(DEFAULT_START) as i32;

        return AutoIncrementProvider {
            start: start_option
        };
    }
}

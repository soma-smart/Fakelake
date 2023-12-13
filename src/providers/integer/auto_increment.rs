use arrow_schema::DataType;
use yaml_rust::Yaml;

use crate::providers::provider::{Provider, Value};
use crate::providers::column_options::ColumnOptions;

const DEFAULT_START: i64 = 0;

pub struct AutoIncrementProvider {
    pub options: Option<ColumnOptions>,
    pub start: i32,
}

impl Provider for AutoIncrementProvider {
    fn value(&self, index: u32) -> Option<Value> {
        let calculated_value = Value::Int32(self.start + (index as i32));
        return match &self.options {
            Some(value) => value.alter_value(calculated_value, index),
            _ => Some(calculated_value),
        }
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Int32;
    }
    fn new_from_yaml(column: &Yaml) -> AutoIncrementProvider {
        let column_options = ColumnOptions::new_from_yaml(column);
        let start_option = column["start"].as_i64().unwrap_or(DEFAULT_START) as i32;

        return AutoIncrementProvider {
            options: column_options,
            start: start_option
        };
    }
}

use arrow_schema::DataType;
use yaml_rust::Yaml;

use std::iter::repeat_with;

use crate::providers::provider::{Provider, Value};
use crate::providers::column_options::ColumnOptions;

const DEFAULT_DOMAIN: &str = "example.com";

pub struct EmailProvider {
    pub options: Option<ColumnOptions>,
    pub domain: String,
}

impl Provider for EmailProvider {
    fn value(&self, index: u32) -> Option<Value> {
        // return a random email address
        // generate a random string of length 10 (subject) + @ + random domain
        let subject: String = repeat_with(fastrand::alphanumeric).take(10).collect();
        let calculated_value = Value::String(format!("{}@{}", subject, self.domain));

        return match &self.options {
            Some(value) => value.alter_value(calculated_value, index),
            _ => Some(calculated_value),
        }
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Utf8;
    }
    fn new_from_yaml(column: &Yaml) -> EmailProvider {
        let column_options = ColumnOptions::new_from_yaml(column);
        let domain_option = column["domain"].as_str().unwrap_or(DEFAULT_DOMAIN).to_string();

        return EmailProvider {
            options: column_options,
            domain: domain_option
        };
    }
}

use arrow_schema::DataType;
use yaml_rust::Yaml;

use std::iter::repeat_with;

use crate::providers::provider::{Provider, Value};

const DEFAULT_DOMAIN: &str = "example.com";

pub struct EmailProvider {
    pub domain: String,
}

impl Provider for EmailProvider {
    fn value(&self, index: u32) -> Value {
        // return a random email address
        // generate a random string of length 10 (subject) + @ + random domain
        let subject: String = repeat_with(fastrand::alphanumeric).take(10).collect();
        return Value::String(format!("{}@{}", subject, self.domain));
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Utf8;
    }
    fn new_from_yaml(column: &Yaml) -> EmailProvider {
        let domain_option = column["domain"].as_str().unwrap_or(DEFAULT_DOMAIN).to_string();

        return EmailProvider {
            domain: domain_option
        };
    }
}

use arrow_schema::DataType;
use yaml_rust::Yaml;

use crate::providers::provider::{Provider, Value};
use super::utils::random_characters;

const DEFAULT_DOMAIN: &str = "example.com";

pub struct EmailProvider {
    pub domain: String,
}

impl Provider for EmailProvider {
    fn value(&self, _: u32) -> Value {
        // return a random email address
        // generate a random string of length 10 (subject) + @ + random domain
        let subject: String = random_characters(10);
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

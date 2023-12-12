use arrow_schema::DataType;
use yaml_rust::Yaml;

use std::iter::repeat_with;

use crate::providers::provider::{Provider, Value};

pub struct EmailProvider {
    pub domain: Option<String>,
}

impl Provider for EmailProvider {
    fn value(&self, index: u32) -> Value {
        // return a random email address
        // generate a random string of length 10 (subject) + @ + random domain
        let subject: String = repeat_with(fastrand::alphanumeric).take(10).collect();
        let domain = match &self.domain {
            Some(domain) => domain,
            None => "example.com",
        };
        return Value::String(format!("{}@{}", subject, domain));
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Utf8;
    }
    fn new_from_yaml(yaml: &Vec<Yaml>) -> EmailProvider {
        return EmailProvider { domain:None };
    }
}

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

#[cfg(test)]
mod tests {
    use crate::providers::provider::{ Value, Provider };
    use super::StringProvider;
    
    // Validate value calculation
    #[test]
    fn given_index_x_should_return_random_string_of_length_10() {
        let provider = StringProvider;

        let values_to_check = [0, 4, 50];
        for value in values_to_check {
            match provider.value(value) {
                Value::String(value) => assert_eq!(value.len(), 10),
                _ => panic!("Wrong type")
            }
        }
    }
}

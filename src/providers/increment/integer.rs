use crate::providers::provider::{Provider, Value};

use arrow_schema::DataType;
use yaml_rust::Yaml;

const DEFAULT_START: i64 = 0;

#[derive(Clone)]
pub struct IncrementIntegerProvider {
    pub start: i32,
}

impl Provider for IncrementIntegerProvider {
    fn value(&self, index: u32) -> Value {
        return Value::Int32(self.start + (index as i32));
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Int32;
    }
    fn new_from_yaml(column: &Yaml) -> IncrementIntegerProvider {
        let start_option = column["start"].as_i64().unwrap_or(DEFAULT_START) as i32;

        return IncrementIntegerProvider {
            start: start_option
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::provider::{ Value, Provider };
    use super::{ DEFAULT_START, IncrementIntegerProvider };

    use arrow_schema::DataType;
    use yaml_rust::YamlLoader;

    fn generate_provider(start: Option<String>) -> IncrementIntegerProvider {
        let yaml_str = match start {
            Some(value) => format!("name: id{}start: {}", "\n", value),
            None => format!("name: id"),
        };
        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        IncrementIntegerProvider::new_from_yaml(&yaml[0])
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider: IncrementIntegerProvider = generate_provider(None);
        assert_eq!(provider.get_parquet_type(), DataType::Int32);
    }

    // Validate YAML file
    #[test]
    fn given_no_start_in_yaml_should_give_default() {
        let provider = generate_provider(None);
        assert_eq!(provider.start, DEFAULT_START as i32);
    }

    #[test]
    fn given_badvalue_for_start_in_yaml_should_give_default() {
        let provider = generate_provider(Some("BadValue".to_string()));
        assert_eq!(provider.start, DEFAULT_START as i32);
    }
    
    #[test]
    fn given_x_for_start_in_yaml_should_give_start_x() {
        let values_to_check = [-14, 0, 4, 50];
        for value in values_to_check {
            let provider = generate_provider(Some(value.to_string()));
            assert_eq!(provider.start, value);
        }
    }
    
    // Validate value calculation
    #[test]
    fn given_start_0_and_index_x_should_return_x() {
        let provider = IncrementIntegerProvider { start: 0 };

        let values_to_check = [0, 4, 50];
        for value in values_to_check {
            let calculated = provider.value(value);
            assert_eq!(calculated, Value::Int32(value as i32));
        }
    }

    #[test]
    fn given_start_x_and_index_y_should_return_x_plus_y() {
        let start_to_check = [-14, 12, 17, 23];
        let values_to_check = [0, 4, 50];

        for start in start_to_check {
            let provider = IncrementIntegerProvider { start };
            for value in values_to_check {
                let calculated = provider.value(value);
                assert_eq!(calculated, Value::Int32(start + value as i32));
            }
        }
    }
}
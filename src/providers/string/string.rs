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

    use arrow_schema::DataType;
    use yaml_rust::YamlLoader;

    fn generate_provider() -> StringProvider {
        let yaml_str = format!("name: id");

        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        StringProvider::new_from_yaml(&yaml[0])
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider: StringProvider = StringProvider;
        assert_eq!(provider.get_parquet_type(), DataType::Utf8);
    }

    // Validate YAML file
    #[test]
    fn given_no_config_should_return_default() {
        let _: StringProvider = generate_provider();
        assert!(true);
    }
    
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

use crate::providers::parameters::urange::URangeParameter;
use crate::providers::provider::{Provider, Value};
use crate::providers::utils::string::random_characters;

use yaml_rust::Yaml;

const DEFAULT_LENGTH: u32 = 10;

#[derive(Clone)]
pub struct AlphanumericProvider {
    pub min_length: u32,
    pub max_length: u32,
}

impl Provider for AlphanumericProvider {
    fn value(&self, _: u32) -> Value {
        Value::String(random_characters(fastrand::u32(
            self.min_length..self.max_length,
        )))
    }
    fn new_from_yaml(column: &Yaml) -> AlphanumericProvider {
        let u_range_parameter = URangeParameter::new(column, "length", DEFAULT_LENGTH);

        AlphanumericProvider {
            min_length: u_range_parameter.min,
            max_length: u_range_parameter.max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AlphanumericProvider;
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider(length: Option<&str>) -> AlphanumericProvider {
        let yaml_length = match length {
            Some(value) => format!("{}length: {}", "\n", value),
            None => String::new(),
        };
        let yaml_str = format!("name: id{}", yaml_length);

        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        AlphanumericProvider::new_from_yaml(&yaml[0])
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_string_type() {
        let provider: AlphanumericProvider = AlphanumericProvider {
            min_length: 10,
            max_length: 11,
        };
        match provider.value(0) {
            Value::String(_) => (),
            _ => panic!(),
        };
    }

    // Validate YAML file
    #[test]
    fn given_no_config_should_return_default() {
        let provider: AlphanumericProvider = generate_provider(None);
        assert_eq!(provider.min_length, 10);
        assert_eq!(provider.max_length, 11);
    }

    #[test]
    fn given_constant_config_should_return_good_length_range() {
        let provider: AlphanumericProvider = generate_provider(Some("8"));
        assert_eq!(provider.min_length, 8);
        assert_eq!(provider.max_length, 9);
    }

    #[test]
    fn given_bad_constant_config_should_return_default() {
        let provider: AlphanumericProvider = generate_provider(Some("test"));
        assert_eq!(provider.min_length, 10);
        assert_eq!(provider.max_length, 11);
    }

    #[test]
    fn given_range_config_should_return_good_length_range() {
        let provider: AlphanumericProvider = generate_provider(Some("8 .. 20"));
        assert_eq!(provider.min_length, 8);
        assert_eq!(provider.max_length, 20);
    }

    #[test]
    fn given_bad_range_config_should_return_default() {
        let provider: AlphanumericProvider = generate_provider(Some("20..8"));
        assert_eq!(provider.min_length, 10);
        assert_eq!(provider.max_length, 11);
    }

    #[test]
    fn given_range_too_big_config_should_return_default() {
        let provider: AlphanumericProvider = generate_provider(Some("20..8..14"));
        assert_eq!(provider.min_length, 10);
        assert_eq!(provider.max_length, 11);
    }

    // Validate value calculation
    #[test]
    fn given_index_x_should_return_random_string_of_length_10() {
        let provider = AlphanumericProvider {
            min_length: 10,
            max_length: 11,
        };

        let values_to_check = [0, 4, 50];
        for value in values_to_check {
            match provider.value(value) {
                Value::String(value) => assert_eq!(value.len(), 10),
                _ => panic!("Wrong type"),
            }
        }
    }
}

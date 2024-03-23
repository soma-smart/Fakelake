use yaml_rust::Yaml;

use crate::providers::parameters::wstring::WStringParameter;
use crate::providers::provider::{Provider, Value};

const DEFAULT_CONSTANT: &str = "constant";

#[derive(Clone)]
pub struct StringProvider {
    data: Vec<WStringParameter>,
}

impl StringProvider {
    fn weighted_random(weighted_strings: Vec<WStringParameter>) -> String {
        let max: u32 = weighted_strings.iter().map(|w| w.weight).sum::<u32>();
        let random: u32 = fastrand::u32(0..max) + 1;
        StringProvider::get_from_weight(weighted_strings, random)
    }

    fn get_from_weight(weighted_strings: Vec<WStringParameter>, random: u32) -> String {
        let mut weight_sum: u32 = 0;
        let mut random_value: Option<String> = None;
        for weighted_string in weighted_strings.into_iter() {
            weight_sum += weighted_string.weight;
            if weight_sum >= random {
                random_value = Some(weighted_string.value);
                break;
            }
        }
        random_value.unwrap_or(DEFAULT_CONSTANT.to_string())
    }
}

impl Provider for StringProvider {
    fn value(&self, _: u32) -> Value {
        Value::String(StringProvider::weighted_random(self.data.to_owned()))
    }
    fn new_from_yaml(column: &Yaml) -> StringProvider {
        let data_option = WStringParameter::new(column, "data", "constant");
        StringProvider { data: data_option }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider(data: Option<&str>) -> StringProvider {
        let yaml_data = match data {
            Some(value) => format!("{}data: {}", "\n", value),
            None => String::new(),
        };
        let yaml_str = format!("name: id{}", yaml_data);
        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        StringProvider::new_from_yaml(&yaml[0])
    }

    #[test]
    fn given_data_as_string_should_return_string_constant_type() {
        let provider = StringProvider {
            data: vec![WStringParameter::new_from_str("my_value")],
        };
        match provider.value(0) {
            Value::String(s) => assert_eq!(s, "my_value"),
            _ => panic!(),
        }
    }

    #[test]
    fn given_data_as_array_should_return_string_constant_randomly_from_array_type() {
        let input: Vec<String> = vec![
            "constant".to_string(),
            "my_data".to_string(),
            "voila".to_string(),
            "my_value".to_string(),
        ];
        let provider: StringProvider = StringProvider {
            data: vec![
                WStringParameter::new_from_str("constant"),
                WStringParameter::new_from_str("my_data"),
                WStringParameter::new_from_str("voila"),
                WStringParameter::new_from_str("my_value"),
            ],
        };
        match provider.value(0) {
            Value::String(s) => assert!(input.contains(&s)),
            _ => panic!(),
        }
    }

    // Validate value calculation
    #[test]
    fn given_no_config_should_return_default() {
        let provider: StringProvider = generate_provider(None);
        assert_eq!(
            provider.data,
            vec![WStringParameter::new_from_str(DEFAULT_CONSTANT)]
        );
    }

    #[test]
    fn given_string_config_should_return_array_value() {
        let provider: StringProvider = generate_provider(Some("my_data"));
        assert_eq!(
            provider.data,
            vec![WStringParameter::new_from_str("my_data")]
        );
    }

    #[test]
    fn given_array_config_should_return_array_value() {
        let input: &str = "[my_data, example, array]";
        let provider: StringProvider = generate_provider(Some(input));
        assert_eq!(
            provider.data,
            vec![
                WStringParameter::new_from_str("my_data"),
                WStringParameter::new_from_str("example"),
                WStringParameter::new_from_str("array")
            ]
        );
    }

    #[test]
    fn given_array_with_weight_config_should_return_array_value() {
        let input: &str =
            "\n  - value: trout \n  - value: salmon \n    weight: 8 \n  - value: carp";
        let provider: StringProvider = generate_provider(Some(input));
        assert_eq!(
            provider.data,
            vec![
                WStringParameter::new_from_str("trout"),
                WStringParameter {
                    value: "salmon".to_string(),
                    weight: 8
                },
                WStringParameter::new_from_str("carp")
            ]
        );
    }

    // Test weighted random
    #[test]
    fn given_random_greater_than_weight_sum_should_return_default_value() {
        let weighted_strings: Vec<WStringParameter> = vec![WStringParameter {
            value: "test".to_string(),
            weight: 1,
        }];
        let result: String = StringProvider::get_from_weight(weighted_strings, 2);
        assert_eq!(DEFAULT_CONSTANT.to_string(), result);
    }

    #[test]
    fn given_random_should_return_corresponding_value() {
        let weighted_strings: Vec<WStringParameter> = vec![
            WStringParameter {
                value: "first".to_string(),
                weight: 1,
            },
            WStringParameter {
                value: "middle".to_string(),
                weight: 8,
            },
            WStringParameter {
                value: "last".to_string(),
                weight: 1,
            },
        ];
        let values = [
            (0, "first"),
            (1, "first"),
            (2, "middle"),
            (9, "middle"),
            (10, "last"),
        ];
        for value in values {
            let result: String =
                StringProvider::get_from_weight(weighted_strings.to_owned(), value.0);
            assert_eq!(value.1.to_owned(), result);
        }
    }
}

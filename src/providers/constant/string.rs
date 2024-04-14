use yaml_rust::Yaml;

use crate::providers::parameters::wstring::WStringParameter;
use crate::providers::provider::{Provider, Value};
use crate::providers::utils::string::random_alphanumeric;

const DEFAULT_CONSTANT: &str = "constant";

#[derive(Clone)]
pub struct ConstantStringProvider {
    data: String,
}

impl Provider for ConstantStringProvider {
    fn value(&self, _: u32) -> Value {
        Value::String(self.data.to_string())
    }

    fn corrupted_value(&self, _: u32) -> Value {
        Value::String(random_alphanumeric(10))
    }
}

#[derive(Clone)]
pub struct ListStringProvider {
    data: Vec<String>,
}

impl Provider for ListStringProvider {
    fn value(&self, _: u32) -> Value {
        let index = fastrand::usize(..self.data.len());
        Value::String(self.data[index].to_string())
    }

    fn corrupted_value(&self, _: u32) -> Value {
        Value::String(random_alphanumeric(10))
    }
}

#[derive(Clone)]
pub struct WeightedListStringProvider {
    data: Vec<WStringParameter>,
    sum: u32,
}

impl WeightedListStringProvider {
    fn new(parameters: Vec<WStringParameter>) -> Self {
        WeightedListStringProvider {
            data: parameters.clone(),
            sum: parameters.into_iter().map(|w| w.weight).sum::<u32>(),
        }
    }

    fn weighted_random(&self) -> String {
        let random: u32 = fastrand::u32(0..self.sum) + 1;
        self.get_from_weight(random)
    }

    fn get_from_weight(&self, random: u32) -> String {
        let mut weight_sum: u32 = 0;
        let mut random_value: Option<String> = None;
        for weighted_string in self.data.iter() {
            weight_sum += weighted_string.weight;
            if weight_sum >= random {
                random_value = Some(weighted_string.value.to_owned());
                break;
            }
        }

        match random_value {
            Some(value) => value,
            None => panic!("Random value can't be empty"),
        }
    }
}

impl Provider for WeightedListStringProvider {
    fn value(&self, _: u32) -> Value {
        Value::String(self.weighted_random())
    }

    fn corrupted_value(&self, _: u32) -> Value {
        Value::String(random_alphanumeric(10))
    }
}

pub fn new_from_yaml(column: &Yaml) -> Box<dyn Provider> {
    let data_option = WStringParameter::new(column, "data", DEFAULT_CONSTANT);
    let length: u32 = data_option.len() as u32;
    if length == 1 {
        Box::new(ConstantStringProvider {
            data: data_option[0].value.to_string(),
        })
    } else {
        let w = WeightedListStringProvider::new(data_option);
        if w.sum == length {
            Box::new(ListStringProvider {
                data: w.data.into_iter().map(|v| v.value).collect(),
            })
        } else {
            Box::new(w)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider(data: Option<&str>) -> Box<dyn Provider> {
        let yaml_data = match data {
            Some(value) => format!("{}data: {}", "\n", value),
            None => String::new(),
        };
        let yaml_str = format!("name: id{}", yaml_data);
        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        super::new_from_yaml(&yaml[0])
    }

    #[test]
    fn given_weighted_list_string_provider_should_compute_weight_sum() {
        let provider: WeightedListStringProvider = WeightedListStringProvider::new(vec![
            WStringParameter {
                value: "constant".to_string(),
                weight: 1,
            },
            WStringParameter {
                value: "my_data".to_string(),
                weight: 2,
            },
            WStringParameter {
                value: "voila".to_string(),
                weight: 3,
            },
            WStringParameter {
                value: "my_value".to_string(),
                weight: 4,
            },
        ]);
        assert_eq!(provider.sum, 10);
    }

    #[test]
    fn given_data_as_string_should_return_string_constant_type() {
        let provider: ConstantStringProvider = ConstantStringProvider {
            data: "my_value".to_string(),
        };
        for i in 0..10 {
            match provider.value(i) {
                Value::String(s) => assert_eq!(s, "my_value"),
                _ => panic!(),
            }
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
        let provider: ListStringProvider = ListStringProvider {
            data: vec![
                "constant".to_string(),
                "my_data".to_string(),
                "voila".to_string(),
                "my_value".to_string(),
            ],
        };
        for i in 0..10 {
            match provider.value(i) {
                Value::String(s) => assert!(input.contains(&s)),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn given_data_as_weighted_list_should_return_string_constant_weighted_randomly() {
        let input: Vec<String> = vec![
            "constant".to_string(),
            "my_data".to_string(),
            "voila".to_string(),
            "my_value".to_string(),
        ];
        let provider: WeightedListStringProvider = WeightedListStringProvider::new(vec![
            WStringParameter {
                value: "constant".to_string(),
                weight: 1,
            },
            WStringParameter {
                value: "my_data".to_string(),
                weight: 1,
            },
            WStringParameter {
                value: "voila".to_string(),
                weight: 1,
            },
            WStringParameter {
                value: "my_value".to_string(),
                weight: 1,
            },
        ]);
        for i in 0..10 {
            match provider.value(i) {
                Value::String(s) => assert!(input.contains(&s)),
                _ => panic!(),
            }
        }
    }

    // Validate value calculation
    #[test]
    fn given_no_config_should_return_default() {
        let provider = generate_provider(None);
        assert_eq!(
            provider.value(0),
            Value::String(DEFAULT_CONSTANT.to_string())
        );
    }

    #[test]
    fn given_string_config_should_return_array_value() {
        let provider = generate_provider(Some("my_data"));
        for i in 0..10 {
            assert_eq!(provider.value(i), Value::String("my_data".to_string()));
        }
    }

    #[test]
    fn given_array_config_should_return_array_value() {
        let expected_input: Vec<String> = vec![
            "my_data".to_string(),
            "example".to_string(),
            "array".to_string(),
        ];
        let input: &str = "[my_data, example, array]";
        let provider = generate_provider(Some(input));
        for i in 0..10 {
            match provider.value(i) {
                Value::String(s) => assert!(expected_input.contains(&s)),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn given_array_with_weight_config_should_return_array_value() {
        let expected_input: Vec<String> = vec![
            "trout".to_string(),
            "salmon".to_string(),
            "carp".to_string(),
        ];
        let input: &str =
            "\n  - value: trout \n  - value: salmon \n    weight: 8 \n  - value: carp";
        let provider = generate_provider(Some(input));
        for i in 0..10 {
            match provider.value(i) {
                Value::String(s) => assert!(expected_input.contains(&s)),
                _ => panic!(),
            }
        }
    }

    // Test weighted random
    #[test]
    #[should_panic(expected = "Random value can't be empty")]
    fn given_random_greater_than_weight_sum_should_return_default_value() {
        let weighted_strings: Vec<WStringParameter> = vec![WStringParameter {
            value: "test".to_string(),
            weight: 1,
        }];
        WeightedListStringProvider::new(weighted_strings).get_from_weight(2);
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
        let w: WeightedListStringProvider = WeightedListStringProvider::new(weighted_strings);
        for value in values {
            let result: String = w.get_from_weight(value.0);
            assert_eq!(value.1.to_owned(), result);
        }
    }

    #[test]
    fn given_weighted_list_string_provider_should_return_values_according_to_weights() {
        let provider: WeightedListStringProvider = WeightedListStringProvider::new(vec![
            WStringParameter {
                value: "one".to_string(),
                weight: 1,
            },
            WStringParameter {
                value: "two".to_string(),
                weight: 6,
            },
            WStringParameter {
                value: "three".to_string(),
                weight: 3,
            },
        ]);

        let mut result: HashMap<String, u32> = HashMap::new();
        for i in 0..1000 {
            match provider.value(i) {
                Value::String(value) => {
                    let count: u32 = result.get(&value).map_or_else(|| 1, |v| v + 1);
                    result.insert(value, count);
                }
                _ => panic!("Should never happened !"),
            };
        }

        let expected = vec![("one", 1), ("two", 6), ("three", 3)];
        for e in expected {
            let v: &u32 = &result
                .get(&e.0.to_string())
                .map_or_else(|| 0, |v| (*v as f64 / 100.0).round() as u32);
            assert_eq!(v, &e.1);
        }
    }

    #[test]
    fn given_no_config_should_return_corrupted_value() {
        let provider = generate_provider(None);
        assert_ne!(
            provider.corrupted_value(0),
            Value::String(DEFAULT_CONSTANT.to_string())
        );
    }

    #[test]
    fn given_string_config_should_return_corrupted_value() {
        let provider = generate_provider(Some("my_data"));
        for i in 0..10 {
            assert_ne!(
                provider.corrupted_value(i),
                Value::String("my_data".to_string())
            );
        }
    }

    #[test]
    fn given_array_config_should_return_corrupted_value() {
        let expected_input: Vec<String> = vec![
            "my_data".to_string(),
            "example".to_string(),
            "array".to_string(),
        ];
        let input: &str = "[my_data, example, array]";
        let provider = generate_provider(Some(input));
        for i in 0..10 {
            match provider.corrupted_value(i) {
                Value::String(s) => assert!(!expected_input.contains(&s)),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn given_array_with_weight_config_should_return_corrupted() {
        let expected_input: Vec<String> = vec![
            "trout".to_string(),
            "salmon".to_string(),
            "carp".to_string(),
        ];
        let input: &str =
            "\n  - value: trout \n  - value: salmon \n    weight: 8 \n  - value: carp";
        let provider = generate_provider(Some(input));
        for i in 0..10 {
            match provider.corrupted_value(i) {
                Value::String(s) => assert!(!expected_input.contains(&s)),
                _ => panic!(),
            }
        }
    }
}

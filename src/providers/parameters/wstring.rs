use linked_hash_map::LinkedHashMap;
use log::warn;
use yaml_rust::Yaml;

use super::get_column_name;

const WEIGHT_KEY: &str = "weight";
const VALUE_KEY: &str = "value";

// WString for weighted string
#[derive(Clone, PartialEq, Debug)]
pub struct WStringParameter {
    pub value: String,
    pub weight: u32,
}

impl WStringParameter {
    pub fn new(column: &Yaml, param_name: &str, default_value: &str) -> Vec<WStringParameter> {
        let column_name = get_column_name(column);
        match &column[param_name] {
            Yaml::Integer(i) => vec![WStringParameter::new_from_i64(i)],
            Yaml::Real(r) => vec![WStringParameter::new_from_str(r)],
            Yaml::String(s) => vec![WStringParameter::new_from_str(s.as_str())],
            Yaml::Array(a) => extract_array(column_name, a, default_value),
            _ => {
                print_wrong_param(
                    column_name,
                    param_name,
                    "string or integer or real or array",
                    default_value,
                );
                vec![WStringParameter::new_from_str(default_value)]
            }
        }
    }

    pub fn new_from_str(value: &str) -> WStringParameter {
        WStringParameter {
            value: value.to_string(),
            weight: 1,
        }
    }

    fn new_from_str_and_weight(
        column_name: &str,
        value: &str,
        weight: Option<i64>,
    ) -> WStringParameter {
        WStringParameter {
            value: value.to_string(),
            weight: extract_weight(column_name, weight),
        }
    }

    fn new_from_i64(value: &i64) -> WStringParameter {
        WStringParameter {
            value: value.to_string(),
            weight: 1,
        }
    }

    fn new_from_i64_and_weight(
        column_name: &str,
        value: &i64,
        weight: Option<i64>,
    ) -> WStringParameter {
        WStringParameter {
            value: value.to_string(),
            weight: extract_weight(column_name, weight),
        }
    }
}

fn extract_weight(column_name: &str, weight: Option<i64>) -> u32 {
    let w: i64 = weight.unwrap_or(1);
    if w >= u32::MIN as i64 && w <= u32::MAX as i64 {
        w as u32
    } else {
        print_wrong_param(column_name, WEIGHT_KEY, "u32", "1");
        1
    }
}

fn extract_array(column_name: &str, array: &[Yaml], default_value: &str) -> Vec<WStringParameter> {
    array
        .iter()
        .map(|i| match i {
            Yaml::Hash(h) => extract_hash(column_name, h, default_value),
            Yaml::String(s) => WStringParameter::new_from_str(s.as_str()),
            Yaml::Integer(i) => WStringParameter::new_from_i64(i),
            Yaml::Real(r) => WStringParameter::new_from_str(r),
            _ => WStringParameter::new_from_str(default_value),
        })
        .collect::<Vec<_>>()
}

fn extract_weight_from_hash(column_name: &str, hash: &LinkedHashMap<Yaml, Yaml>) -> Option<i64> {
    hash.get(&Yaml::String(String::from(WEIGHT_KEY)))
        .and_then(|w| match w {
            Yaml::Integer(i) => Some(i.to_owned()),
            _ => {
                print_wrong_param(column_name, WEIGHT_KEY, "u32", "1");
                None
            }
        })
}

fn extract_hash(
    column_name: &str,
    hash: &LinkedHashMap<Yaml, Yaml>,
    default_value: &str,
) -> WStringParameter {
    let weight = extract_weight_from_hash(column_name, hash);
    match hash
        .get(&Yaml::String(String::from(VALUE_KEY)))
        .map(|f| match f {
            Yaml::String(s) => WStringParameter::new_from_str_and_weight(column_name, s, weight),
            Yaml::Real(r) => WStringParameter::new_from_str_and_weight(column_name, r, weight),
            Yaml::Integer(i) => WStringParameter::new_from_i64_and_weight(column_name, i, weight),
            _ => {
                print_wrong_param(
                    column_name,
                    VALUE_KEY,
                    "string or integer or real",
                    default_value,
                );
                WStringParameter::new_from_str(default_value)
            }
        }) {
        Some(value) => value,
        None => panic!("Yaml value is invalid"),
    }
}

fn print_wrong_param(column_name: &str, param_name: &str, value_type: &str, new_value: &str) {
    warn!(
        "Column {} param {} should be {}. Value {} is taken instead.",
        column_name, param_name, value_type, new_value
    );
}

#[cfg(test)]
mod tests {
    use linked_hash_map::LinkedHashMap;
    use yaml_rust::{Yaml, YamlLoader};

    use crate::providers::parameters::wstring::extract_weight;
    use crate::providers::parameters::wstring::extract_weight_from_hash;
    use crate::providers::parameters::wstring::WStringParameter;

    use super::extract_hash;
    use super::WEIGHT_KEY;

    fn generate_yaml(name: Option<&str>, param_name: &str, param_value: Option<&str>) -> Vec<Yaml> {
        let yaml_name = match name {
            Some(value) => format!("name: {}{}", value, "\n"),
            None => String::new(),
        };

        let yaml_param = match param_value {
            Some(value) => format!("{}: {}", param_name, value),
            None => String::new(),
        };

        let yaml_str = format!("{}{}", yaml_name, yaml_param);
        YamlLoader::load_from_str(yaml_str.as_str()).unwrap()
    }

    #[test]
    fn given_correct_float_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("15.5"));
        let wstringparameters = WStringParameter::new(&yaml_param[0], "param", "constant");
        assert_eq!(
            wstringparameters,
            vec![WStringParameter {
                value: "15.5".to_string(),
                weight: 1
            }]
        );
    }

    #[test]
    fn given_correct_u32_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("15"));
        let wstringparameters = WStringParameter::new(&yaml_param[0], "param", "constant");
        assert_eq!(
            wstringparameters,
            vec![WStringParameter {
                value: "15".to_string(),
                weight: 1
            }]
        );
    }

    #[test]
    fn given_correct_u32_as_string_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("\"15\""));
        let wstringparameters = WStringParameter::new(&yaml_param[0], "param", "constant");
        assert_eq!(
            wstringparameters,
            vec![WStringParameter {
                value: "15".to_string(),
                weight: 1
            }]
        );
    }

    #[test]
    fn given_correct_string_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("salmon"));
        let wstringvecparameters = WStringParameter::new(&yaml_param[0], "param", "constant");
        assert_eq!(
            wstringvecparameters,
            vec![WStringParameter {
                value: "salmon".to_string(),
                weight: 1
            }]
        );
    }

    #[test]
    fn given_correct_array_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("[salmon, 15, 20.55]"));
        let wstringvecparameters = WStringParameter::new(&yaml_param[0], "param", "constant");
        assert_eq!(
            wstringvecparameters,
            vec![
                WStringParameter {
                    value: "salmon".to_string(),
                    weight: 1
                },
                WStringParameter {
                    value: "15".to_string(),
                    weight: 1
                },
                WStringParameter {
                    value: "20.55".to_string(),
                    weight: 1
                }
            ]
        );
    }

    #[test]
    fn given_correct_list_param_should_give_value() {
        let yaml_param = generate_yaml(
            Some("col"),
            "param",
            Some("\n  - value: salmon \n  - value: 15 \n  - value: 13.6"),
        );
        let wstringvecparameters = WStringParameter::new(&yaml_param[0], "param", "constant");
        assert_eq!(
            wstringvecparameters,
            vec![
                WStringParameter {
                    value: "salmon".to_string(),
                    weight: 1
                },
                WStringParameter {
                    value: "15".to_string(),
                    weight: 1
                },
                WStringParameter {
                    value: "13.6".to_string(),
                    weight: 1
                }
            ]
        );
    }

    #[test]
    fn given_correct_weighted_list_param_should_give_value() {
        let yaml_param = generate_yaml(
            Some("col"),
            "param",
            Some("\n  - value: 15 \n  - value: salmon \n    weight: 8 \n  - value: 12.3"),
        );
        let wstringvecparameters = WStringParameter::new(&yaml_param[0], "param", "constant");
        assert_eq!(
            wstringvecparameters,
            vec![
                WStringParameter {
                    value: "15".to_string(),
                    weight: 1
                },
                WStringParameter {
                    value: "salmon".to_string(),
                    weight: 8
                },
                WStringParameter {
                    value: "12.3".to_string(),
                    weight: 1
                }
            ]
        );
    }

    #[test]
    fn given_weight_should_return_u32() {
        let data: [(i64, u32); 6] = [
            (0, 0),
            (-1, 1),
            (u32::MAX as i64, u32::MAX),
            (u32::MAX as i64 + 1, 1),
            (u32::MIN as i64, u32::MIN),
            (u32::MIN as i64 - 1, 1),
        ];
        for d in data {
            let w = extract_weight("col", Some(d.0));
            assert_eq!(w, d.1);
        }
    }

    #[test]
    #[should_panic(expected = "Yaml value is invalid")]
    fn given_empty_hash_should_panic_extract_hash() {
        let hash = LinkedHashMap::new();
        extract_hash("column_name", &hash, "default_value");
    }

    #[test]
    fn given_wrong_weight_should_return_none() {
        let mut hash = LinkedHashMap::new();
        hash.insert(
            Yaml::String(WEIGHT_KEY.to_string()),
            Yaml::String("bad".to_string()),
        );
        let result = extract_weight_from_hash("col", &hash);
        assert_eq!(None, result);
    }
}

use log::warn;
use yaml_rust::Yaml;

use super::get_column_name;

pub struct URangeParameter {
    pub min: u32,
    pub max: u32,
}

impl URangeParameter {
    pub fn new(column: &Yaml, param_name: &str, default_value: u32) -> URangeParameter {
        let column_name = get_column_name(column);

        let u32parameters = match &column[param_name] {
            Yaml::Integer(value) => new_from_i64(column_name, param_name, *value, default_value),
            Yaml::String(value) => new_from_range(column_name, param_name, value, default_value),
            Yaml::BadValue => {
                vec![default_value, default_value + 1]
            }
            _ => {
                print_wrong_param(column_name, param_name, default_value);
                vec![default_value, default_value + 1]
            }
        };

        URangeParameter {
            min: u32parameters[0],
            max: u32parameters[1],
        }
    }
}

fn new_from_i64(column_name: &str, param_name: &str, value: i64, default_value: u32) -> Vec<u32> {
    let u32_value = if value < 0 || value >= u32::MAX as i64 {
        print_wrong_param(column_name, param_name, default_value);
        default_value
    } else {
        value as u32
    };

    vec![u32_value, u32_value + 1]
}

fn new_from_range(
    column_name: &str,
    param_name: &str,
    value: &str,
    default_value: u32,
) -> Vec<u32> {
    let split_range: Vec<&str> = value.split("..").collect();
    match split_range.len() {
        1 => match split_range[0].parse::<i64>() {
            Ok(value) => new_from_i64(column_name, param_name, value, default_value),
            _ => {
                print_wrong_param(column_name, param_name, default_value);
                vec![default_value, default_value + 1]
            }
        },
        2 => {
            let mut wrong_param = false;
            let left = match split_range[0].trim().parse::<i64>() {
                Ok(value) => new_from_i64(column_name, param_name, value, default_value)[0],
                _ => {
                    wrong_param = true;
                    default_value
                }
            };

            let right = match split_range[1].trim().parse::<i64>() {
                Ok(value) => new_from_i64(column_name, param_name, value, default_value)[0],
                _ => {
                    wrong_param = true;
                    default_value
                }
            };

            if wrong_param {
                print_wrong_param(column_name, param_name, default_value);
                return vec![default_value, default_value + 1];
            }

            if left < right {
                vec![left, right]
            } else {
                print_wrong_param(column_name, param_name, default_value);
                vec![default_value, default_value + 1]
            }
        }
        _ => {
            print_wrong_param(column_name, param_name, default_value);
            vec![default_value, default_value + 1]
        }
    }
}

fn print_wrong_param(column_name: &str, param_name: &str, new_value: u32) {
    warn!(
        "Column {} param {} should be an u32 or increasing u32..u32. Value {}..{} is taken instead.",
        column_name,
        param_name,
        new_value,
        new_value + 1
    );
}

#[cfg(test)]
mod tests {
    use yaml_rust::{Yaml, YamlLoader};

    use super::URangeParameter;

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
    fn given_correct_u32_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("15"));
        let urangeparameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(urangeparameter.min, 15);
        assert_eq!(urangeparameter.max, 16);
    }

    #[test]
    fn given_correct_u32_as_string_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("\"15\""));
        let urangeparameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(urangeparameter.min, 15);
        assert_eq!(urangeparameter.max, 16);
    }

    #[test]
    fn given_correct_range_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("15..18"));
        let urangeparameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(urangeparameter.min, 15);
        assert_eq!(urangeparameter.max, 18);
    }

    #[test]
    fn given_no_param_should_give_default() {
        let yaml_param = generate_yaml(Some("col"), "param", None);
        let i32parameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(i32parameter.min, 10);
        assert_eq!(i32parameter.max, 11);
    }

    #[test]
    fn given_incorrect_negative_i32_param_should_give_default_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("-12"));
        let urangeparameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(urangeparameter.min, 10);
        assert_eq!(urangeparameter.max, 11);
    }

    #[test]
    fn given_incorrect_f32_param_should_give_default_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("0.7"));
        let urangeparameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(urangeparameter.min, 10);
        assert_eq!(urangeparameter.max, 11);
    }

    #[test]
    fn given_incorrect_i64_param_should_give_default_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some(&i64::MAX.to_string()));
        let urangeparameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(urangeparameter.min, 10);
        assert_eq!(urangeparameter.max, 11);
    }

    #[test]
    fn given_incorrect_first_range_should_give_default_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("a..15"));
        let urangeparameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(urangeparameter.min, 10);
        assert_eq!(urangeparameter.max, 11);
    }

    #[test]
    fn given_incorrect_second_range_should_give_default_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("5..a"));
        let urangeparameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(urangeparameter.min, 10);
        assert_eq!(urangeparameter.max, 11);
    }

    #[test]
    fn given_incorrect_three_range_should_give_default_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("5..6..10"));
        let urangeparameter = URangeParameter::new(&yaml_param[0], "param", 10);
        assert_eq!(urangeparameter.min, 10);
        assert_eq!(urangeparameter.max, 11);
    }
}

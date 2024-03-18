use yaml_rust::Yaml;

use log::warn;

use super::get_column_name;

pub struct I32Parameter {
    pub value: i32,
}

impl I32Parameter {
    pub fn new(column: &Yaml, param_name: &str, default_value: i32) -> I32Parameter {
        let column_name = get_column_name(column);

        let param_i64 = match column[param_name] {
            Yaml::Integer(value) => value,
            Yaml::BadValue => default_value as i64,
            _ => {
                print_wrong_param(column_name, param_name, default_value);
                default_value as i64
            }
        };

        let param_i32 = if param_i64 < i32::MIN as i64 || param_i64 > i32::MAX as i64 {
            print_wrong_param(column_name, param_name, default_value);
            default_value
        } else {
            param_i64 as i32
        };

        I32Parameter { value: param_i32 }
    }
}

fn print_wrong_param(column_name: &str, param_name: &str, new_value: i32) {
    warn!(
        "Column {} param {} should be an i32. Value {} is taken instead.",
        column_name, param_name, new_value
    );
}

#[cfg(test)]
mod tests {
    use yaml_rust::{Yaml, YamlLoader};

    use super::I32Parameter;

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
    fn given_correct_i32_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("1000"));
        let i32parameter = I32Parameter::new(&yaml_param[0], "param", 500);
        assert_eq!(i32parameter.value, 1000);
    }

    #[test]
    fn given_no_i32_param_should_give_default() {
        let yaml_param = generate_yaml(Some("col"), "param", None);
        let i32parameter = I32Parameter::new(&yaml_param[0], "param", 500);
        assert_eq!(i32parameter.value, 500);
    }

    #[test]
    fn given_string_for_i32_param_should_give_default() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("str"));
        let i32parameter = I32Parameter::new(&yaml_param[0], "param", 500);
        assert_eq!(i32parameter.value, 500);
    }

    #[test]
    fn given_i64_for_i32_param_should_give_default() {
        let yaml_param = generate_yaml(Some("col"), "param", Some(&i64::MAX.to_string()));
        let i32parameter = I32Parameter::new(&yaml_param[0], "param", 500);
        assert_eq!(i32parameter.value, 500);
    }
}

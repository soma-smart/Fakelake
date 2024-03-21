use yaml_rust::Yaml;

use log::warn;

use super::get_column_name;

pub struct F64Parameter {
    pub value: f64,
}

impl F64Parameter {
    pub fn new(column: &Yaml, param_name: &str, default_value: f64) -> F64Parameter {
        let column_name = get_column_name(column);

        let param_f64 = match &column[param_name] {
            Yaml::Real(value) => value.parse::<f64>().unwrap(),
            Yaml::Integer(value) => *value as f64,
            Yaml::BadValue => default_value,
            _ => {
                print_wrong_param(column_name, param_name, default_value);
                default_value
            }
        };

        F64Parameter { value: param_f64 }
    }
}

fn print_wrong_param(column_name: &str, param_name: &str, new_value: f64) {
    warn!(
        "Column {} param {} should be an f64. Value {} is taken instead.",
        column_name, param_name, new_value
    );
}

#[cfg(test)]
mod tests {
    use yaml_rust::{Yaml, YamlLoader};

    use super::F64Parameter;

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
    fn given_i64_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("5"));
        let f64parameter = F64Parameter::new(&yaml_param[0], "param", 10.4);
        assert_eq!(f64parameter.value, 5.0);
    }

    #[test]
    fn given_correct_f64_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("5.3"));
        let f64parameter = F64Parameter::new(&yaml_param[0], "param", 10.4);
        assert_eq!(f64parameter.value, 5.3);
    }

    #[test]
    fn given_no_f64_param_should_give_default() {
        let yaml_param = generate_yaml(Some("col"), "param", None);
        let f64parameter = F64Parameter::new(&yaml_param[0], "param", 10.4);
        assert_eq!(f64parameter.value, 10.4);
    }

    #[test]
    fn given_string_for_f64_param_should_give_default() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("str"));
        let f64parameter = F64Parameter::new(&yaml_param[0], "param", 10.4);
        assert_eq!(f64parameter.value, 10.4);
    }
}

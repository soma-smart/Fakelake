use yaml_rust::Yaml;

use log::warn;

use super::get_column_name;

pub struct StringParameter {
    pub value: String,
}

impl StringParameter {
    pub fn new(column: &Yaml, param_name: &str, default_value: &str) -> StringParameter {
        let column_name = get_column_name(column);
        let param_str = match &column[param_name] {
            Yaml::String(value) => value,
            Yaml::BadValue => default_value,
            _ => {
                print_wrong_param(column_name, param_name, default_value);
                default_value
            }
        };

        StringParameter {
            value: param_str.to_string(),
        }
    }
}

fn print_wrong_param(column_name: &str, param_name: &str, new_value: &str) {
    warn!(
        "Column {} param {} should be a string. Value {} is taken instead.",
        column_name, param_name, new_value
    );
}

#[cfg(test)]
mod tests {
    use yaml_rust::{Yaml, YamlLoader};

    use super::StringParameter;

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
    fn given_correct_str_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("value.fr"));
        let strparameter = StringParameter::new(&yaml_param[0], "param", "soma-smart.com");
        assert_eq!(strparameter.value, "value.fr");
    }

    #[test]
    fn given_no_str_param_should_give_default() {
        let yaml_param = generate_yaml(Some("col"), "param", None);
        let strparameter = StringParameter::new(&yaml_param[0], "param", "soma-smart.com");
        assert_eq!(strparameter.value, "soma-smart.com");
    }

    #[test]
    fn given_number_for_str_param_should_give_default() {
        let yaml_param = generate_yaml(Some("col"), "param", Some("3"));
        let strparameter = StringParameter::new(&yaml_param[0], "param", "soma-smart.com");
        assert_eq!(strparameter.value, "soma-smart.com");
    }
}

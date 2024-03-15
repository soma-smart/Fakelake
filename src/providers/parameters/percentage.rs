use yaml_rust::Yaml;

use log::warn;

use super::get_column_name;

pub struct PercentageParameter {
    pub value: f64,
}

fn parameter_to_f64(column: &Yaml, param_name: &str, default_value: f64) -> Result<f64, ()> {
    match &column[param_name] {
        Yaml::Integer(value) => Ok(*value as f64),
        Yaml::Real(value) => Ok(value.parse::<f64>().unwrap()),
        Yaml::BadValue => Ok(default_value),
        _ => Err(()),
    }
}

impl PercentageParameter {
    pub fn new(column: &Yaml, param_name: &str, default_value: f64) -> PercentageParameter {
        let column_name = get_column_name(column);

        #[allow(clippy::needless_late_init)]
        let percent_value;
        match parameter_to_f64(column, param_name, default_value) {
            Ok(value) => {
                percent_value = value;
            }
            Err(_) => return wrap_up_issue(column_name, param_name, default_value),
        }

        match percent_value {
            value if value < 0.0 => wrap_up_issue(column_name, param_name, 0.0),
            value if value > 1.0 => wrap_up_issue(column_name, param_name, 1.0),
            value => PercentageParameter { value },
        }
    }
}

fn wrap_up_issue(column_name: &str, param_name: &str, new_value: f64) -> PercentageParameter {
    warn!(
        "Column {} param {} should be between 0 and 1. Value {} is taken instead.",
        column_name, param_name, new_value
    );
    PercentageParameter { value: new_value }
}

#[cfg(test)]
mod tests {
    use yaml_rust::YamlLoader;

    use super::PercentageParameter;

    fn generate_yaml(
        name: Option<&str>,
        param_name: &str,
        param_value: Option<&str>,
    ) -> PercentageParameter {
        let yaml_name = match name {
            Some(value) => format!("name: {}{}", value, "\n"),
            None => String::new(),
        };

        let yaml_param = match param_value {
            Some(value) => format!("{}: {}", param_name, value),
            None => String::new(),
        };

        let yaml_str = format!("{}{}", yaml_name, yaml_param);
        let column = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        PercentageParameter::new(&column[0], param_name, 1.0)
    }

    #[test]
    fn given_no_value_should_give_default() {
        let param = generate_yaml(Some("name"), "presence", None);
        assert_eq!(param.value, 1.0);
    }

    #[test]
    fn given_less_than_0_int_should_give_0() {
        let param = generate_yaml(Some("name"), "presence", Some("-5"));
        assert_eq!(param.value, 0.0);
    }

    #[test]
    fn given_less_than_0_float_should_give_0() {
        let param = generate_yaml(Some("name"), "presence", Some("-5.2"));
        assert_eq!(param.value, 0.0);
    }

    #[test]
    fn given_more_than_1_int_should_give_1() {
        let param = generate_yaml(Some("name"), "presence", Some("5"));
        assert_eq!(param.value, 1.0);
    }

    #[test]
    fn given_more_than_1_float_should_give_1() {
        let param = generate_yaml(Some("name"), "presence", Some("5.2"));
        assert_eq!(param.value, 1.0);
    }

    #[test]
    fn given_bad_value_should_give_default() {
        let param = generate_yaml(Some("name"), "presence", Some("BadValue"));
        assert_eq!(param.value, 1.0);
    }

    #[test]
    fn given_0_int_should_give_0() {
        let param = generate_yaml(Some("name"), "presence", Some("0"));
        assert_eq!(param.value, 0.0);
    }

    #[test]
    fn given_0_float_should_give_0() {
        let param = generate_yaml(Some("name"), "presence", Some("0.0"));
        assert_eq!(param.value, 0.0);
    }

    #[test]
    fn given_1_int_should_give_1() {
        let param = generate_yaml(Some("name"), "presence", Some("1"));
        assert_eq!(param.value, 1.0);
    }

    #[test]
    fn given_1_float_should_give_1() {
        let param = generate_yaml(Some("name"), "presence", Some("1.0"));
        assert_eq!(param.value, 1.0);
    }

    #[test]
    fn given_between_0_and_1_should_give_same_value() {
        let param = generate_yaml(Some("name"), "presence", Some("0.7"));
        assert_eq!(param.value, 0.7);
    }
}

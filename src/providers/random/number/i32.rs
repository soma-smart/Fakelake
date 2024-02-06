use crate::providers::provider::{Provider, Value};

use log::warn;
use yaml_rust::Yaml;

const DEFAULT_MIN: i32 = i32::MIN;
const DEFAULT_MAX: i32 = i32::MAX;

#[derive(Clone)]
pub struct I32Provider {
    pub min: i32,
    pub max: i32,
}

impl Provider for I32Provider {
    fn value(&self, _: u32) -> Value {
        Value::Int32(fastrand::i32(self.min..self.max))
    }
    fn new_from_yaml(column: &Yaml) -> I32Provider {
        let yaml_min = column["min"].as_i64().unwrap_or(DEFAULT_MIN.into());
        let yaml_max = column["max"].as_i64().unwrap_or(DEFAULT_MAX.into());
        let mut param_min: i32 = DEFAULT_MIN;
        let mut param_max: i32 = DEFAULT_MAX;

        if yaml_min >= i32::MIN as i64 && yaml_min <= i32::MAX as i64 {
            param_min = yaml_min as i32
        } else {
            warn!(
                "min option is not an i32. Default ({}) is taken.",
                DEFAULT_MIN
            )
        }
        if yaml_max >= i32::MIN as i64 && yaml_max <= i32::MAX as i64 {
            param_max = yaml_max as i32
        } else {
            warn!(
                "max option is not an i32. Default ({}) is taken.",
                DEFAULT_MAX
            )
        }

        if param_min >= param_max {
            warn!(
                "min option is not less or equal to max option. Default are used ([{} and {}[)",
                DEFAULT_MIN, DEFAULT_MAX
            );
            param_min = DEFAULT_MIN;
            param_max = DEFAULT_MAX;
        }

        I32Provider {
            min: param_min,
            max: param_max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{I32Provider, DEFAULT_MAX, DEFAULT_MIN};
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider(min: Option<&str>, max: Option<&str>) -> I32Provider {
        let yaml_min = match min {
            Some(value) => format!("{}min: {}", "\n", value),
            None => String::new(),
        };
        let yaml_max = match max {
            Some(value) => format!("{}max: {}", "\n", value),
            None => String::new(),
        };

        let yaml_str = format!("name: id{}{}", yaml_min, yaml_max);

        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        I32Provider::new_from_yaml(&yaml[0])
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider: I32Provider = generate_provider(None, None);
        match provider.value(0) {
            Value::Int32(_) => (),
            _ => panic!(),
        };
    }

    // Validate yaml config
    #[test]
    fn given_no_params_should_use_default() {
        let provider: I32Provider = generate_provider(None, None);

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, DEFAULT_MAX);
    }

    #[test]
    fn given_normal_params_should_use_params() {
        let provider: I32Provider = generate_provider(Some("-100"), Some("100"));

        assert_eq!(provider.min, -100);
        assert_eq!(provider.max, 100);
    }

    #[test]
    fn given_no_max_param_should_use_default() {
        let provider: I32Provider = generate_provider(Some("-100"), None);

        assert_eq!(provider.min, -100);
        assert_eq!(provider.max, DEFAULT_MAX);
    }

    #[test]
    fn given_no_min_param_should_use_default() {
        let provider: I32Provider = generate_provider(None, Some("100"));

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, 100);
    }

    #[test]
    fn given_too_small_min_param_should_use_default() {
        let provider: I32Provider = generate_provider(Some(&i64::MIN.to_string()), Some("100"));

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, 100);
    }

    #[test]
    fn given_too_big_max_param_should_use_default() {
        let provider: I32Provider = generate_provider(Some("-100"), Some(&i64::MAX.to_string()));

        assert_eq!(provider.min, -100);
        assert_eq!(provider.max, DEFAULT_MAX);
    }

    #[test]
    fn given_inverted_min_max_params_should_use_default() {
        let provider: I32Provider = generate_provider(Some("100"), Some("-100"));

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, DEFAULT_MAX);
    }
}

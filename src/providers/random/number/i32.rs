use crate::providers::parameters::get_column_name;
use crate::providers::parameters::i32::I32Parameter;
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
    fn corrupted_value(&self, _: u32) -> Value {
        Value::Int32(fastrand::i32(i32::MIN..i32::MAX))
    }
}

pub fn new_from_yaml(column: &Yaml) -> Box<I32Provider> {
    let yaml_min = I32Parameter::new(column, "min", DEFAULT_MIN).value;
    let yaml_max = I32Parameter::new(column, "max", DEFAULT_MAX).value;

    if yaml_min >= yaml_max {
        warn!(
            "Column {} min is not less or equal to max option. Default are used ([{} and {}[)",
            get_column_name(column),
            DEFAULT_MIN,
            DEFAULT_MAX
        );
        Box::new(I32Provider {
            min: DEFAULT_MIN,
            max: DEFAULT_MAX,
        })
    } else {
        Box::new(I32Provider {
            min: yaml_min,
            max: yaml_max,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{I32Provider, DEFAULT_MAX, DEFAULT_MIN};
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider(min: Option<&str>, max: Option<&str>) -> Box<I32Provider> {
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
        super::new_from_yaml(&yaml[0])
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider = generate_provider(None, None);
        match provider.value(0) {
            Value::Int32(_) => (),
            _ => panic!(),
        };
    }

    // Validate yaml config
    #[test]
    fn given_no_params_should_use_default() {
        let provider = generate_provider(None, None);

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, DEFAULT_MAX);
    }

    #[test]
    fn given_normal_params_should_use_params() {
        let provider = generate_provider(Some("-100"), Some("100"));

        assert_eq!(provider.min, -100);
        assert_eq!(provider.max, 100);
    }

    #[test]
    fn given_no_max_param_should_use_default() {
        let provider = generate_provider(Some("-100"), None);

        assert_eq!(provider.min, -100);
        assert_eq!(provider.max, DEFAULT_MAX);
    }

    #[test]
    fn given_no_min_param_should_use_default() {
        let provider = generate_provider(None, Some("100"));

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, 100);
    }

    #[test]
    fn given_too_small_min_param_should_use_default() {
        let provider = generate_provider(Some(&i64::MIN.to_string()), Some("100"));

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, 100);
    }

    #[test]
    fn given_too_big_max_param_should_use_default() {
        let provider = generate_provider(Some("-100"), Some(&i64::MAX.to_string()));

        assert_eq!(provider.min, -100);
        assert_eq!(provider.max, DEFAULT_MAX);
    }

    #[test]
    fn given_inverted_min_max_params_should_use_default() {
        let provider = generate_provider(Some("100"), Some("-100"));

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, DEFAULT_MAX);
    }

    #[test]
    fn given_small_interval_should_corrupted_return_random() {
        let provider = generate_provider(Some("-100"), Some("100"));

        let mut count_random_int = 0;
        for i in 0..100 {
            let value = match provider.corrupted_value(i) {
                Value::Int32(res) => res,
                _ => panic!("Should not happen"),
            };

            if !(-100..=100).contains(&value) {
                count_random_int += 1;
            }
        }

        assert!(count_random_int >= 99);
    }
}

use crate::providers::parameters::f64::F64Parameter;
use crate::providers::parameters::get_column_name;
use crate::providers::provider::{Provider, Value};

use log::warn;
use yaml_rust::Yaml;

const DEFAULT_MIN: f64 = f64::MIN;
const DEFAULT_MAX: f64 = f64::MAX;

#[derive(Clone)]
pub struct F64Provider {
    pub min: f64,
    pub max: f64,
}

impl Provider for F64Provider {
    fn value(&self, _: u32) -> Value {
        Value::Float64(fastrand_contrib::f64_range(self.min..self.max))
    }
    fn corrupted_value(&self, _: u32) -> Value {
        Value::Float64(fastrand_contrib::f64_range(f64::MIN..f64::MAX))
    }
}

pub fn new_from_yaml(column: &Yaml) -> Box<F64Provider> {
    let yaml_min = F64Parameter::new(column, "min", DEFAULT_MIN).value;
    let yaml_max = F64Parameter::new(column, "max", DEFAULT_MAX).value;

    if yaml_min >= yaml_max {
        warn!(
            "Column {} min is not less or equal to max option. Default are used ([{} and {}[)",
            get_column_name(column),
            DEFAULT_MIN,
            DEFAULT_MAX
        );
        Box::new(F64Provider {
            min: DEFAULT_MIN,
            max: DEFAULT_MAX,
        })
    } else {
        Box::new(F64Provider {
            min: yaml_min,
            max: yaml_max,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{F64Provider, DEFAULT_MAX, DEFAULT_MIN};
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider(min: Option<&str>, max: Option<&str>) -> Box<F64Provider> {
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
            Value::Float64(_) => (),
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
        let provider = generate_provider(Some("-10.3"), Some("10.5"));

        assert_eq!(provider.min, -10.3);
        assert_eq!(provider.max, 10.5);
    }

    #[test]
    fn given_no_max_param_should_use_default() {
        let provider = generate_provider(Some("-40.6"), None);

        assert_eq!(provider.min, -40.6);
        assert_eq!(provider.max, DEFAULT_MAX);
    }

    #[test]
    fn given_no_min_param_should_use_default() {
        let provider = generate_provider(None, Some("40.3"));

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, 40.3);
    }

    #[test]
    fn given_inverted_min_max_params_should_use_default() {
        let provider = generate_provider(Some("14.6"), Some("-24.5"));

        assert_eq!(provider.min, DEFAULT_MIN);
        assert_eq!(provider.max, DEFAULT_MAX);
    }

    #[test]
    fn given_small_interval_should_corrupted_return_random() {
        let provider = generate_provider(Some("-10.5"), Some("14.7"));

        let mut count_random_float = 0;
        for i in 0..100 {
            let value = match provider.corrupted_value(i) {
                Value::Float64(res) => res,
                _ => panic!("Should not happen"),
            };

            if !(-10.5..=14.7).contains(&value) {
                count_random_float += 1;
            }
        }

        assert!(count_random_float >= 99);
    }
}

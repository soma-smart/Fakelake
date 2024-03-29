use crate::providers::parameters::i32::I32Parameter;
use crate::providers::provider::{Provider, Value};

use fastrand;
use yaml_rust::Yaml;

const DEFAULT_START: i32 = 0;
const DEFAULT_STEP: i32 = 1;

#[derive(Clone)]
pub struct IncrementIntegerProvider {
    pub start: i32,
    pub step: i32,
}

impl Provider for IncrementIntegerProvider {
    fn value(&self, index: u32) -> Value {
        Value::Int32(self.start + ((index as i32) * self.step))
    }
    fn corrupted_value(&self, _: u32) -> Value {
        // return random i32
        Value::Int32(fastrand::i32(i32::MIN..i32::MAX))
    }
}

pub fn new_from_yaml(column: &Yaml) -> Box<IncrementIntegerProvider> {
    let start_param = I32Parameter::new(column, "start", DEFAULT_START);
    let step_param = I32Parameter::new(column, "step", DEFAULT_STEP);

    Box::new(IncrementIntegerProvider {
        start: start_param.value,
        step: step_param.value,
    })
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::{IncrementIntegerProvider, DEFAULT_START, DEFAULT_STEP};
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider(start: Option<&str>, step: Option<&str>) -> Box<IncrementIntegerProvider> {
        let yaml_start = match start {
            Some(value) => format!("{}start: {}", "\n", value),
            None => String::new(),
        };
        let yaml_step = match step {
            Some(value) => format!("{}step: {}", "\n", value),
            None => String::new(),
        };

        let yaml_str = format!("name: id{}{}", yaml_start, yaml_step);

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

    // Validate YAML file
    #[test]
    fn given_yaml_should_give_integer_provider() {
        let provider = generate_provider(Some("10"), Some("2"));
        assert_eq!(provider.start, 10);
        assert_eq!(provider.step, 2);
    }

    #[test]
    fn given_no_start_and_no_step_in_yaml_should_give_defaults() {
        let provider = generate_provider(None, None);
        assert_eq!(provider.start, DEFAULT_START);
        assert_eq!(provider.step, DEFAULT_STEP);
    }

    #[test]
    fn given_badvalue_for_start_and_step_in_yaml_should_give_defaults() {
        let provider = generate_provider(Some("BadValue"), Some("BadValue"));
        assert_eq!(provider.start, DEFAULT_START);
        assert_eq!(provider.step, DEFAULT_STEP);
    }

    #[test]
    fn given_i64_for_start_and_step_in_yaml_should_give_defaults() {
        let provider = generate_provider(Some(&i64::MIN.to_string()), Some(&i64::MAX.to_string()));
        assert_eq!(provider.start, DEFAULT_START);
        assert_eq!(provider.step, DEFAULT_STEP);
    }

    #[test]
    fn given_x_for_start_and_y_for_step_in_yaml_should_give_start_x_and_step_y() {
        let start_to_check = [-14, 0, 4, 50];
        let step_to_check = [-10, 0, 1, 3, 20];
        for start in start_to_check {
            for step in step_to_check {
                let provider = generate_provider(Some(&start.to_string()), Some(&step.to_string()));
                assert_eq!(provider.start, start);
                assert_eq!(provider.step, step);
            }
        }
    }

    // Validate value calculation
    #[test]
    fn given_start_0_and_index_x_and_step_1_should_return_x() {
        let provider = IncrementIntegerProvider { start: 0, step: 1 };

        let values_to_check = [0, 4, 50];
        for value in values_to_check {
            let calculated = provider.value(value);
            assert_eq!(calculated, Value::Int32(value as i32));
        }
    }

    #[test]
    fn given_start_x_and_index_y_and_step_1_should_return_x_plus_y() {
        let start_to_check = [-14, 12, 17, 23];
        let values_to_check = [0, 4, 50];

        for start in start_to_check {
            let provider = IncrementIntegerProvider { start, step: 1 };
            for value in values_to_check {
                let calculated = provider.value(value);
                assert_eq!(calculated, Value::Int32(start + value as i32));
            }
        }
    }

    #[test]
    fn given_start_x_and_index_y_and_step_z_should_return_x_plus_y_multiply_z() {
        let start_to_check = [-14, 12, 17, 23];
        let step_to_check = [-4, -1, 0, 1, 2, 3, 5];
        let values_to_check = [0, 4, 50];

        for start in start_to_check {
            for step in step_to_check {
                let provider = IncrementIntegerProvider { start, step };
                for value in values_to_check {
                    let calculated = provider.value(value);
                    assert_eq!(calculated, Value::Int32(start + (value as i32 * step)));
                }
            }
        }
    }

    // Corrupted value
    #[test]
    fn given_increment_integer_provider_should_corrupted_return_random_int() {
        let start_to_check = [-14, 12, 17, 23];
        let step_to_check = [-4, -1, 0, 1, 2, 3, 5];
        let values_to_check = [0, 4, 50];

        for start in start_to_check {
            for step in step_to_check {
                let provider = IncrementIntegerProvider { start, step };

                let mut is_random_int = false;
                for value in values_to_check {
                    let calculated = match provider.corrupted_value(value) {
                        Value::Int32(res) => res,
                        _ => panic!("Should not happen"),
                    };
                    is_random_int =
                        calculated < start || calculated >= (start + value as i32 * step);
                }
                assert!(is_random_int)
            }
        }
    }
}

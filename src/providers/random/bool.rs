use crate::providers::provider::{Provider, Value};

use yaml_rust::Yaml;

#[derive(Clone)]
pub struct BoolProvider {}

impl Provider for BoolProvider {
    fn value(&self, _: u32) -> Value {
        Value::Bool(fastrand::bool())
    }
    fn corrupted_value(&self, index: u32) -> Value {
        // Corrupted boolean is not valid
        self.value(index)
    }
}

pub fn new_from_yaml(_: &Yaml) -> Box<BoolProvider> {
    Box::new(BoolProvider {})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider() -> Box<BoolProvider> {
        let yaml_str = "name: is_present".to_string();

        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        super::new_from_yaml(&yaml[0])
    }

    // Validate YAML file
    #[test]
    fn given_nothing_should_return_bool_type() {
        let provider = BoolProvider {};
        match provider.value(0) {
            Value::Bool(_) => (),
            _ => panic!(),
        }
    }

    // Validate value calculation
    #[test]
    fn given_no_config_should_return_default() {
        let _ = generate_provider();
    }

    #[test]
    fn given_provider_should_corrupted_return_default() {
        let provider = generate_provider();
        for i in 0..100 {
            match provider.corrupted_value(i) {
                Value::Bool(_) => continue,
                _ => panic!("Should not happen"),
            };
        }
    }
}

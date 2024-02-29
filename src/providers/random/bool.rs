use crate::providers::provider::{Provider, Value};

use yaml_rust::Yaml;

#[derive(Clone)]
pub struct BoolProvider {}

impl Provider for BoolProvider {
    fn value(&self, _: u32) -> Value {
        Value::Bool(fastrand::bool())
    }
    fn new_from_yaml(_: &Yaml) -> BoolProvider {
        BoolProvider {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider() -> BoolProvider {
        let yaml_str = "name: is_present".to_string();

        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        BoolProvider::new_from_yaml(&yaml[0])
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
}

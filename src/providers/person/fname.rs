use crate::providers::{
    provider::{Provider, Value},
    utils::string::random_characters,
};

use once_cell::sync::Lazy;
use yaml_rust::Yaml;

static FIRST_NAMES: Lazy<Vec<&str>> = Lazy::new(|| {
    let raw_first_names = include_str!("../../../static/first_name_fr.txt");
    raw_first_names.lines().collect()
});

#[derive(Clone)]
pub struct FirstNameProvider {}

impl Provider for FirstNameProvider {
    fn value(&self, _: u32) -> Value {
        let index = fastrand::usize(..FIRST_NAMES.len());
        Value::String(FIRST_NAMES[index].to_string())
    }
    fn corrupted_value(&self, _: u32) -> Value {
        Value::String(random_characters(10))
    }
}

pub fn new_from_yaml(_: &Yaml) -> Box<FirstNameProvider> {
    Box::new(FirstNameProvider {})
}

#[cfg(test)]
mod tests {
    use super::{FirstNameProvider, FIRST_NAMES};
    use crate::providers::provider::{Provider, Value};

    use yaml_rust::YamlLoader;

    fn generate_provider() -> Box<FirstNameProvider> {
        let yaml_str = "name: id".to_string();
        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        super::new_from_yaml(&yaml[0])
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider = generate_provider();
        match provider.value(0) {
            Value::String(_) => (),
            _ => panic!(),
        };
    }

    // Validate value calculation
    #[test]
    fn given_nothing_should_return_name_from_list() {
        let provider = generate_provider();
        match provider.value(0) {
            Value::String(value) => assert!(FIRST_NAMES.contains(&value.as_str())),
            _ => panic!("Wrong type"),
        }
    }

    #[test]
    fn given_nothing_should_corrupted_return_name_not_from_list() {
        let provider = generate_provider();
        match provider.corrupted_value(0) {
            Value::String(value) => assert!(!FIRST_NAMES.contains(&value.as_str())),
            _ => panic!("Wrong type"),
        }
    }
}

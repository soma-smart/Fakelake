use crate::providers::provider::{Provider, Value};

use once_cell::sync::Lazy;
use yaml_rust::Yaml;

static FIRST_NAMES: Lazy<Vec<&str>> = Lazy::new(|| {
    let RAW_FIRST_NAMES = include_str!("../../../static/first_name_fr.txt");
    RAW_FIRST_NAMES.lines().collect()
});

#[derive(Clone)]
pub struct FirstNameProvider {}

impl Provider for FirstNameProvider {
    fn value(&self, _: u32) -> Value {
        let index = fastrand::usize(..FIRST_NAMES.len());
        return Value::String(FIRST_NAMES[index].to_string());
    }
    fn new_from_yaml(_: &Yaml) -> FirstNameProvider {
        return FirstNameProvider {};
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::provider::{ Value, Provider };
    use super::{ FIRST_NAMES, FirstNameProvider };

    use yaml_rust::YamlLoader;

    fn generate_provider() -> FirstNameProvider {
        let yaml_str = format!("name: id");
        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        FirstNameProvider::new_from_yaml(&yaml[0])
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider: FirstNameProvider = generate_provider();
        match provider.value(0) {
            Value::String(_) => assert!(true),
            _ => assert!(false)
        };
    }
    
    // Validate value calculation
    #[test]
    fn given_nothing_should_return_name_from_list() {
        let provider: FirstNameProvider = generate_provider();
        match provider.value(0) {
            Value::String(value) => assert!(FIRST_NAMES.contains(&value.as_str())),
            _ => panic!("Wrong type")
        }
    }
}
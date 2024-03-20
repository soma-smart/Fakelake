use crate::errors::FakeLakeError;
use crate::providers::provider::Provider;

use super::f64::F64Provider;
use super::i32::I32Provider;

use yaml_rust::Yaml;

pub fn get_corresponding_provider(
    mut provider_split: std::str::Split<'_, char>,
    column: &Yaml,
) -> Result<Box<dyn Provider>, FakeLakeError> {
    match provider_split.next() {
        Some("i32") => Ok(Box::new(I32Provider::new_from_yaml(column))),
        Some("f64") => Ok(Box::new(F64Provider::new_from_yaml(column))),
        _ => Err(FakeLakeError::BadYAMLFormat("".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::get_corresponding_provider;

    use yaml_rust::YamlLoader;

    #[test]
    fn given_i32_should_return_provider() {
        let provider_name = "i32";
        let yaml_str = format!("name: random_int{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split('.');
        match get_corresponding_provider(provider_split, column) {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_f64_should_return_provider() {
        let provider_name = "f64";
        let yaml_str = format!("name: random_float{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split('.');
        match get_corresponding_provider(provider_split, column) {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_wrong_provider_should_return_error() {
        let provider_name = "not_a_provider";
        let yaml_str = format!("name: not{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split('.');
        match get_corresponding_provider(provider_split, column) {
            Err(_) => (),
            _ => panic!(),
        }
    }
}

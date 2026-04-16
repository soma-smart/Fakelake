use yaml_rust::Yaml;

use crate::errors::FakeLakeError;
use crate::providers::provider::{unknown_provider, Provider};

use super::{external, string};

const AVAILABLE: &[&str] = &["constant.string", "constant.external"];

pub fn get_corresponding_provider(
    mut provider_split: std::str::Split<'_, char>,
    column: &Yaml,
) -> Result<Box<dyn Provider>, FakeLakeError> {
    match provider_split.next() {
        Some("string") => Ok(string::new_from_yaml(column)),
        Some("external") => Ok(external::new_from_yaml(column)),
        other => Err(unknown_provider(
            &format!("constant.{}", other.unwrap_or("<missing>")),
            AVAILABLE,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::get_corresponding_provider;

    use yaml_rust::YamlLoader;

    #[test]
    fn given_valid_provider_should_return_provider() {
        let provider_names = vec!["string", "external"];

        for provider_name in provider_names {
            let yaml_str = format!(
                "name: is_suscribed{}provider: {}{}path: {}",
                '\n', provider_name, '\n', "static/first_name_fr.txt"
            );
            let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

            let provider_split = provider_name.split('.');
            match get_corresponding_provider(provider_split, column) {
                Ok(_) => (),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn given_wrong_provider_should_return_error() {
        let provider_name = "not_a_provider";
        let yaml_str = format!("name: email{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split('.');
        match get_corresponding_provider(provider_split, column) {
            Err(_) => (),
            _ => panic!(),
        }
    }
}

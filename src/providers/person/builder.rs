use crate::errors::FakeLakeError;
use crate::providers::provider::Provider;

use super::email::EmailProvider;
use super::fname::FirstNameProvider;

use yaml_rust::Yaml;

pub fn get_corresponding_provider(mut provider_split: std::str::Split<'_, &str>, column: &Yaml) -> Result<Box<dyn Provider>, FakeLakeError> {
    match provider_split.next() {
        Some("email") => Ok(Box::new(EmailProvider::new_from_yaml(&column))),
        Some("fname") => Ok(Box::new(FirstNameProvider::new_from_yaml(&column))),
        _ => Err(FakeLakeError::BadYAMLFormat("".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::get_corresponding_provider;

    use yaml_rust::YamlLoader;

    #[test]
    fn given_email_should_return_provider() {
        let provider_name = "email";
        let yaml_str = format!("name: email{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split(".");
        match get_corresponding_provider(provider_split, column) {
            Ok(_) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn given_fname_should_return_provider() {
        let provider_name = "fname";
        let yaml_str = format!("name: fname{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split(".");
        match get_corresponding_provider(provider_split, column) {
            Ok(_) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn given_wrong_provider_should_return_error() {
        let provider_name = "not_a_provider";
        let yaml_str = format!("name: email{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split(".");
        match get_corresponding_provider(provider_split, column) {
            Err(_) => assert!(true),
            _ => assert!(false)
        }
    }
}
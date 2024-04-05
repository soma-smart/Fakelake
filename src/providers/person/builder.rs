use crate::errors::FakeLakeError;
use crate::providers::constant::external;
use crate::providers::provider::Provider;

use super::email;

use once_cell::sync::Lazy;
use yaml_rust::Yaml;

static FIRST_NAMES: Lazy<Vec<String>> = Lazy::new(|| {
    let raw_first_names = include_str!("../../../static/first_name_fr.txt");
    raw_first_names.lines().map(|v| v.to_string()).collect()
});

static LAST_NAMES: Lazy<Vec<String>> = Lazy::new(|| {
    let raw_larst_names = include_str!("../../../static/last_name_fr.txt");
    raw_larst_names.lines().map(|v| v.to_string()).collect()
});

pub fn get_corresponding_provider(
    mut provider_split: std::str::Split<'_, char>,
    column: &Yaml,
) -> Result<Box<dyn Provider>, FakeLakeError> {
    match provider_split.next() {
        Some("email") => Ok(email::new_from_yaml(column)),
        Some("fname") => Ok(external::new(FIRST_NAMES.to_vec())),
        Some("lname") => Ok(external::new(LAST_NAMES.to_vec())),
        _ => Err(FakeLakeError::BadYAMLFormat("".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::person::builder::LAST_NAMES;

    use super::{get_corresponding_provider, FIRST_NAMES};

    use yaml_rust::YamlLoader;

    #[test]
    fn given_email_should_return_provider() {
        let provider_name = "email";
        let yaml_str = format!("name: email{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split('.');
        match get_corresponding_provider(provider_split, column) {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_fname_should_return_provider() {
        let provider_name = "fname";
        let yaml_str = format!("name: fname{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split('.');
        match get_corresponding_provider(provider_split, column) {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_lname_should_return_provider() {
        let provider_name = "lname";
        let yaml_str = format!("name: lname{}provider: {}", '\n', provider_name);
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
        let yaml_str = format!("name: email{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let provider_split = provider_name.split('.');
        match get_corresponding_provider(provider_split, column) {
            Err(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_name_files_should_be_loaded() {
        assert_eq!(13387, FIRST_NAMES.len());
        assert_eq!(95590, LAST_NAMES.len());
    }
}

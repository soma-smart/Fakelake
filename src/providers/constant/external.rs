use yaml_rust::Yaml;

use crate::providers::{generic::lstring::ListStringProvider, parameters::file::FileParameter};

pub fn new_from_yaml(column: &Yaml) -> Box<ListStringProvider> {
    let file_parameter: FileParameter = FileParameter::new(column, "path");
    new(file_parameter.get_file_content())
}

pub fn new(values: Vec<String>) -> Box<ListStringProvider> {
    Box::new(ListStringProvider::new(values))
}

#[cfg(test)]
mod tests {
    use crate::providers::{
        parameters::file::FileParameter,
        provider::{CloneProvider, Value},
    };
    use yaml_rust::{Yaml, YamlLoader};

    use super::{new, new_from_yaml};

    fn generate_yaml(name: Option<&str>, param_name: &str, param_value: Option<&str>) -> Vec<Yaml> {
        let yaml_name = match name {
            Some(value) => format!("name: {}{}", value, "\n"),
            None => String::new(),
        };

        let yaml_param = match param_value {
            Some(value) => format!("{}: {}", param_name, value),
            None => String::new(),
        };

        let yaml_str = format!("{}{}", yaml_name, yaml_param);
        YamlLoader::load_from_str(yaml_str.as_str()).unwrap()
    }

    #[test]
    fn given_file_should_get_content() {
        let expected = [
            "test".to_string(),
            "external".to_string(),
            "".to_string(),
            "data".to_string(),
        ];
        let provider = new(FileParameter {
            path: "tests/example.txt".to_string(),
        }
        .get_file_content());
        match provider.clone_box().value(0) {
            Value::String(value) => assert!(expected.contains(&value.to_owned())),
            _ => panic!("Error"),
        }
    }

    #[test]
    fn given_file_should_get_content_from_yaml() {
        let expected = [
            "test".to_string(),
            "external".to_string(),
            "".to_string(),
            "data".to_string(),
        ];
        let yaml_param = generate_yaml(Some("col"), "path", Some("tests/example.txt"));
        let provider = new_from_yaml(&yaml_param[0]);
        match provider.clone_box().value(0) {
            Value::String(value) => assert!(expected.contains(&value.to_owned())),
            _ => panic!("Error"),
        }
    }

    #[test]
    #[should_panic]
    fn given_not_existing_file_should_panic() {
        let yaml_param = generate_yaml(Some("col"), "path", Some("toto.txt"));
        new_from_yaml(&yaml_param[0]);
    }
}

use std::fs::read_to_string;

use yaml_rust::Yaml;

use super::get_column_name;

pub struct FileParameter {
    pub path: String,
}

impl FileParameter {
    pub fn new(column: &Yaml, param_name: &str) -> Self {
        let column_name = get_column_name(column);
        let param_str = match &column[param_name] {
            Yaml::String(value) => value,
            _ => {
                panic!(
                    "Column {} param {} should be a string and can't be null or empty",
                    column_name, param_name
                )
            }
        };

        FileParameter {
            path: param_str.to_string(),
        }
    }

    pub fn get_file_content(&self) -> Vec<String> {
        match read_to_string(&self.path) {
            Ok(content) => content.lines().map(|v| v.to_string()).collect(),
            Err(error) => panic!("Error {} occured when read file {}", error, &self.path),
        }
    }
}

#[cfg(test)]
mod tests {
    use yaml_rust::{Yaml, YamlLoader};

    use super::FileParameter;

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
    fn given_correct_str_param_should_give_value() {
        let yaml_param = generate_yaml(Some("col"), "path", Some("/path/to/file.txt"));
        let file_parameter = FileParameter::new(&yaml_param[0], "path");
        assert_eq!(file_parameter.path, "/path/to/file.txt");
    }

    #[test]
    #[should_panic]
    fn given_no_str_param_should_panic() {
        let yaml_param = generate_yaml(Some("col"), "path", None);
        FileParameter::new(&yaml_param[0], "path");
    }

    #[test]
    #[should_panic]
    fn given_int_param_should_panic() {
        let yaml_param = generate_yaml(Some("col"), "path", Some("2"));
        FileParameter::new(&yaml_param[0], "path");
    }

    #[test]
    #[should_panic]
    fn given_empty_string_param_should_panic() {
        let yaml_param = generate_yaml(Some("col"), "path", Some(""));
        FileParameter::new(&yaml_param[0], "path");
    }

    #[test]
    #[should_panic]
    fn given_not_existing_file_should_panic() {
        let yaml_param = generate_yaml(Some("col"), "path", Some("toto.txt"));
        let file_parameter = FileParameter::new(&yaml_param[0], "path");
        file_parameter.get_file_content();
    }

    #[test]
    fn given_file_should_get_content() {
        let yaml_param = generate_yaml(Some("col"), "path", Some("tests/example.txt"));
        let file_parameter = FileParameter::new(&yaml_param[0], "path");
        let content = file_parameter.get_file_content();
        assert_eq!(content, vec!["test", "external", "", "data"])
    }
}

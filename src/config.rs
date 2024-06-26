/// Config structs used by Fakelake during YAML parsing
///
use log::warn;
use yaml_rust::{Yaml, YamlLoader};

use crate::errors::FakeLakeError;
use crate::options::presence;
use crate::providers::provider::{CorruptedProvider, Provider, ProviderBuilder};

#[derive(Debug)]
pub struct Config {
    pub columns: Vec<Column>,
    pub info: Option<Info>,
}

impl Config {
    pub fn get_output_file_name(&self, extension: &str) -> String {
        let file_name = match &self.info {
            Some(info) => match &info.output_name {
                Some(name) => name,
                None => "output",
            },
            None => "output",
        };

        if file_name.contains(extension) {
            warn!("output_name parameter contains the file extension. It has not been added a second time.");
            return file_name.to_string();
        }

        format!("{}{}", file_name, extension)
    }

    pub fn get_number_of_rows(&self) -> u32 {
        match &self.info {
            Some(info) => info.rows.unwrap_or(1_000_000),
            None => 1_000_000,
        }
    }
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub provider: Box<dyn Provider>,
    pub presence: Box<dyn presence::Presence>,
}

impl Clone for Column {
    fn clone(&self) -> Self {
        Column {
            name: self.name.clone(),
            provider: self.provider.clone_box(),
            presence: self.presence.clone_box(),
        }
    }
}

impl Column {
    pub fn is_next_present(&self) -> bool {
        self.presence.is_next_present()
    }
    pub fn can_be_null(&self) -> bool {
        self.presence.can_be_null()
    }

    pub fn generate_columns(parsed_yaml: &[Yaml]) -> Result<Vec<Column>, FakeLakeError> {
        let mut columns = Vec::new();

        let yaml_columns = match parsed_yaml.first() {
            Some(value) => match value["columns"].as_vec() {
                Some(value) => value,
                None => {
                    return Err(FakeLakeError::BadYAMLFormat(
                        "No columns found in the yaml file".to_string(),
                    ))
                }
            },
            None => {
                return Err(FakeLakeError::BadYAMLFormat(
                    "The yaml file is empty".to_string(),
                ))
            }
        };

        // iter over columns
        for column in yaml_columns {
            let name = match column["name"].as_str() {
                Some(value) => value,
                None => {
                    return Err(FakeLakeError::BadYAMLFormat(
                        "One column in the yaml as no name specified.".to_string(),
                    ))
                }
            };
            let provider = match column["provider"].as_str() {
                Some(value) => value,
                None => {
                    return Err(FakeLakeError::BadYAMLFormat(
                        "The column {{name}} in the yaml as no provider specified.".to_string(),
                    ))
                }
            };

            let presence = presence::new_from_yaml(column);

            let provider: Box<dyn Provider> =
                match ProviderBuilder::get_corresponding_provider(provider, column) {
                    Ok(value) => CorruptedProvider::new_from_yaml(column, value),
                    Err(e) => return Err(FakeLakeError::BadYAMLFormat(e.to_string())),
                };

            let column = Column {
                name: name.to_string(),
                provider,
                presence,
            };
            columns.push(column);
        }

        Ok(columns)
    }
}

#[derive(Debug, PartialEq)]
pub enum OutputType {
    Parquet(),
    Csv(u8),
    Json(bool),
}

#[derive(Debug)]
pub struct Info {
    /// If not specified, output_name takes the name of the input file
    pub output_name: Option<String>,
    /// By default, output_format is Parquet
    pub output_format: Option<OutputType>,
    pub rows: Option<u32>,
}

impl Info {
    pub fn parse_info_section(parsed_yaml: &[Yaml]) -> Result<Info, FakeLakeError> {
        let section_info = match parsed_yaml.first() {
            Some(value) => &value["info"],
            None => {
                return Err(FakeLakeError::BadYAMLFormat(
                    "Yaml file is empty".to_string(),
                ))
            }
        };

        let output_name = section_info["output_name"]
            .as_str()
            .map(|name| name.to_string());

        let output_format = match section_info["output_format"]
            .as_str()
            .map(|format| format.to_string())
        {
            Some(value) if value == "parquet" => Some(OutputType::Parquet()),
            Some(value) if value == "csv" => {
                let delimiter = match section_info["delimiter"].as_str() {
                    Some(delimiter_param) => {
                        match delimiter_param.len() {
                            value if value > 1 => {
                                warn!("Delimiter given for CSV should be one char. Default ',' is taken.");
                                b','
                            }
                            1 => *delimiter_param.as_bytes().first().unwrap(),
                            _ => b',',
                        }
                    }
                    _ => b',',
                };
                Some(OutputType::Csv(delimiter))
            }
            Some(value) if value == "json" => {
                let wrap_up = match section_info["wrap_up"] {
                    Yaml::Boolean(value) => value,
                    Yaml::BadValue => false,
                    _ => {
                        warn!("Wrap up should be a bool. Default value 'false' is taken.");
                        false
                    }
                };
                Some(OutputType::Json(wrap_up))
            }
            _ => None,
        };

        // rows could be i64 or str (i64 with _ separators)
        let rows = match section_info["rows"].as_i64() {
            Some(rows) => Some(rows as u32),
            None => match section_info["rows"].as_str() {
                Some(rows) => match rows.replace('_', "").parse::<u32>() {
                    Ok(value) => Some(value),
                    Err(_) => None,
                },
                None => None,
            },
        };

        Ok(Info {
            output_name,
            output_format,
            rows,
        })
    }
}

pub fn get_config_from_string(file_content: String) -> Result<Config, FakeLakeError> {
    let parsed_yaml = match YamlLoader::load_from_str(&file_content) {
        Ok(docs) => docs,
        Err(e) => return Err(FakeLakeError::BadYAMLFormat(e.to_string())),
    };

    let columns = match Column::generate_columns(&parsed_yaml) {
        Ok(value) => value,
        Err(e) => return Err(FakeLakeError::BadYAMLFormat(e.to_string())),
    };

    let info = Info::parse_info_section(&parsed_yaml).unwrap();

    let config = Config {
        columns,
        info: Some(info),
    };

    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::options::presence::Presence;
    use crate::providers::provider::{Provider, Value};

    use super::*;

    use mockall::predicate::*;
    use mockall::*;

    #[allow(dead_code)]
    #[derive(Clone)]
    struct TestProvider;
    mock! {
        pub TestProvider {}

        impl Clone for TestProvider {
            fn clone(&self) -> Self;
        }

        impl Provider for TestProvider {
            fn value(&self, index: u32) -> Value;
            fn corrupted_value(&self, index: u32) -> Value;
        }
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    struct TestPresence;
    mock! {
        pub TestPresence {}

        impl Clone for TestPresence {
            fn clone(&self) -> Self;
        }

        impl Presence for TestPresence {
            fn is_next_present(&self) -> bool;
            fn can_be_null(&self) -> bool;
        }
    }

    fn generate_column(provider: Box<dyn Provider>, presence: Box<dyn Presence>) -> Column {
        Column {
            name: "Testing column".to_string(),
            provider,
            presence,
        }
    }

    fn expecting_err<T, E>(res: &Result<T, E>) {
        match res {
            Err(_value) => (),
            _ => panic!(),
        }
    }

    fn expecting_ok<T, E>(res: &Result<T, E>) {
        match res {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    // Test Column
    #[test]
    fn given_column_should_call_is_next_present() {
        let mock_provider = Box::new(MockTestProvider::new());

        let mut mock_presence = Box::new(MockTestPresence::new());
        mock_presence
            .expect_is_next_present()
            .times(1)
            .return_const(true);

        let column = generate_column(mock_provider, mock_presence);
        assert!(column.is_next_present());
    }

    #[test]
    fn given_column_should_call_can_be_null() {
        let mock_provider = Box::new(MockTestProvider::new());

        let mut mock_presence = Box::new(MockTestPresence::new());
        mock_presence
            .expect_can_be_null()
            .times(1)
            .return_const(true);

        let column = generate_column(mock_provider, mock_presence);
        assert!(column.can_be_null());
    }

    fn generate_columns_from_yaml(yaml_str: &str) -> Result<Vec<Column>, FakeLakeError> {
        let yaml = YamlLoader::load_from_str(yaml_str).unwrap();

        Column::generate_columns(&yaml)
    }

    // Generate Columns
    #[test]
    fn given_empty_config_should_columns_return_err() {
        let yaml = "";
        let columns = generate_columns_from_yaml(yaml);
        expecting_err(&columns);
    }

    #[test]
    fn given_no_columns_should_columns_return_err() {
        let yaml = "something:\n";
        let columns = generate_columns_from_yaml(yaml);
        expecting_err(&columns);
    }

    #[test]
    fn given_empty_columns_should_columns_return_err() {
        let yaml = "columns:\n";
        let columns = generate_columns_from_yaml(yaml);
        expecting_err(&columns);
    }

    #[test]
    fn given_one_column_without_name_should_columns_return_err() {
        let yaml = "
        columns:
            - provider: Increment.integer
        ";
        let columns = generate_columns_from_yaml(yaml);
        expecting_err(&columns);
    }

    #[test]
    fn given_one_column_without_provider_should_columns_return_err() {
        let yaml = "
        columns:
            - name: id
        ";
        let columns = generate_columns_from_yaml(yaml);
        expecting_err(&columns);
    }

    #[test]
    fn given_unknown_provider_should_columns_return_err() {
        let yaml = "
        columns:
            - name: id
              provider: not-existing-provider
        ";
        let columns = generate_columns_from_yaml(yaml);
        expecting_err(&columns);
    }

    #[test]
    fn given_correct_column_should_columns_return_ok() {
        let yaml = "
        columns:
            - name: id
              provider: Increment.integer
        ";
        let columns = generate_columns_from_yaml(yaml);
        expecting_ok(&columns);
    }

    fn generate_info_from_yaml(yaml_str: &str) -> Result<Info, FakeLakeError> {
        let yaml = YamlLoader::load_from_str(yaml_str).unwrap();

        Info::parse_info_section(&yaml)
    }

    // Generate config
    #[test]
    fn given_empty_yaml_should_config_return_err() {
        let yaml = "";
        let info = generate_info_from_yaml(yaml);
        expecting_err(&info);
    }

    #[test]
    fn given_no_info_should_config_return_ok() {
        let yaml = "something:\n";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
    }

    #[test]
    fn given_empty_info_should_config_return_ok() {
        let yaml = "info: something\n";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
    }

    #[test]
    fn given_no_parameters_should_config_return_ok() {
        let yaml = "
        info:
            a: something
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, None);
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_output_name_should_config_return_in_output_name() {
        let yaml = "
        info:
            output_name: something
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, Some("something".to_string()));
        assert_eq!(info.output_format, None);
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_output_format_should_config_return_in_output_format() {
        let yaml = "
        info:
            output_format: parquet
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, Some(OutputType::Parquet()));
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_csv_format_should_use_default_delimiter() {
        let yaml = "
        info:
            output_format: csv
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, Some(OutputType::Csv(b',')));
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_csv_format_with_custom_delimiter_should_use_custom_delimiter() {
        let yaml = "
        info:
            output_format: csv
            delimiter: '|'
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, Some(OutputType::Csv(b'|')));
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_csv_format_with_too_long_delimiter_should_use_default_delimiter() {
        let yaml = "
        info:
            output_format: csv
            delimiter: '|||'
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, Some(OutputType::Csv(b',')));
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_csv_format_with_empty_delimiter_should_use_default_delimiter() {
        let yaml = "
        info:
            output_format: csv
            delimiter: ''
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, Some(OutputType::Csv(b',')));
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_json_format_should_use_default_wrap_up() {
        let yaml = "
        info:
            output_format: json
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, Some(OutputType::Json(false)));
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_json_format_with_custom_wrap_up_should_use_custom_wrap_up() {
        let yaml = "
        info:
            output_format: json
            wrap_up: true
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, Some(OutputType::Json(true)));
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_json_format_with_invalid_wrap_up_should_default_wrap_up() {
        let yaml = "
        info:
            output_format: json
            wrap_up: invalid
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, Some(OutputType::Json(false)));
        assert_eq!(info.rows, None);
    }

    #[test]
    fn given_rows_int_should_config_return_in_rows() {
        let yaml = "
        info:
            rows: 1000000
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, None);
        assert_eq!(info.rows, Some(1000000));
    }

    #[test]
    fn given_rows_str_should_config_return_in_rows() {
        let yaml = "
        info:
            rows: 1_000_000
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, None);
        assert_eq!(info.rows, Some(1000000));
    }

    #[test]
    fn given_rows_bad_str_should_config_return_none_rows() {
        let yaml = "
        info:
            rows: not_an_int
        ";
        let info = generate_info_from_yaml(yaml);
        expecting_ok(&info);
        let info = &info.unwrap();
        assert_eq!(info.output_name, None);
        assert_eq!(info.output_format, None);
        assert_eq!(info.rows, None);
    }

    // get_config_from_string
    #[test]
    fn given_not_yaml_should_return_err() {
        let file_content = "{{::,..,@#|\"".to_string();
        let res = get_config_from_string(file_content);
        expecting_err(&res);
    }

    #[test]
    fn given_empty_string_should_return_err() {
        let file_content = "".to_string();
        let res = get_config_from_string(file_content);
        expecting_err(&res);
    }

    #[test]
    fn given_only_columns_should_return_ok() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer
        "
        .to_string();
        let res = get_config_from_string(file_content);
        expecting_ok(&res);
    }

    #[test]
    fn given_only_info_should_return_err() {
        let file_content = "
        info:
            output_name: something
            output_format: parquet
            rows: 1000000        
        "
        .to_string();
        let res = get_config_from_string(file_content);
        expecting_err(&res);
    }

    #[test]
    fn given_everything_should_return_ok() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer

        info:
            output_name: something
            output_format: parquet
            rows: 1000000    
        "
        .to_string();
        let res = get_config_from_string(file_content);
        expecting_ok(&res);
    }

    // get_output_file_name
    #[test]
    fn given_no_info_should_return_default_name() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer
        "
        .to_string();
        let config = get_config_from_string(file_content).unwrap();
        assert_eq!(config.get_output_file_name(".parquet"), "output.parquet");
    }

    #[test]
    fn given_no_name_should_return_default() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer
        info:
            output_format: parquet
            rows: 1000
        "
        .to_string();
        let config = get_config_from_string(file_content).unwrap();
        assert_eq!(config.get_output_file_name(".parquet"), "output.parquet");
    }

    #[test]
    fn given_name_should_return_name() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer
        info:
            output_name: expected_name
            output_format: parquet
            rows: 1000
        "
        .to_string();
        let config = get_config_from_string(file_content).unwrap();
        assert_eq!(
            config.get_output_file_name(".parquet"),
            "expected_name.parquet"
        );
    }

    #[test]
    fn given_no_extension_should_return_name() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer
        info:
            output_name: expected_name
            output_format: parquet
            rows: 1000
        "
        .to_string();
        let config = get_config_from_string(file_content).unwrap();
        assert_eq!(config.get_output_file_name(""), "expected_name");
    }

    #[test]
    fn given_file_name_with_extension_should_return_name() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer
        info:
            output_name: expected_name.parquet
            output_format: parquet
            rows: 1000
        "
        .to_string();
        let config = get_config_from_string(file_content).unwrap();
        assert_eq!(
            config.get_output_file_name(".parquet"),
            "expected_name.parquet"
        );
    }

    // get_output_rows
    #[test]
    fn given_no_info_should_return_default_rows() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer
        "
        .to_string();
        let config = get_config_from_string(file_content).unwrap();
        assert_eq!(config.get_number_of_rows(), 1_000_000);
    }

    #[test]
    fn given_no_rows_should_return_default() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer
        info:
            output_name: output
            output_format: parquet
        "
        .to_string();
        let config = get_config_from_string(file_content).unwrap();
        assert_eq!(config.get_number_of_rows(), 1_000_000);
    }

    #[test]
    fn given_rows_should_return_rows() {
        let file_content = "
        columns:
            - name: id
              provider: Increment.integer
        info:
            output_name: expected_name
            output_format: parquet
            rows: 1_000
        "
        .to_string();
        let config = get_config_from_string(file_content).unwrap();
        assert_eq!(config.get_number_of_rows(), 1_000);
    }
}

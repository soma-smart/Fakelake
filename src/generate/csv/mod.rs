use crate::config::Config;
use crate::errors::FakeLakeError;
use crate::generate::output_format::OutputFormat;
use crate::providers::provider::Value;

use csv::Writer;

const CSV_EXTENSION: &str = ".csv";

#[derive(Debug, PartialEq)]
pub struct OutputCsv;

impl OutputFormat for OutputCsv {
    fn get_extension(&self) -> &str {
        CSV_EXTENSION
    }

    fn generate_from_config(&self, config: &Config) -> Result<(), FakeLakeError> {
        if config.columns.is_empty() {
            return Err(FakeLakeError::BadYAMLFormat(
                "No columns to generate".to_string(),
            ));
        }

        let file_name = config.get_output_file_name();
        let rows = config.get_number_of_rows();

        let mut wtr = match Writer::from_path(format!("{}{}", file_name, self.get_extension())) {
            Ok(value) => value,
            Err(e) => {
                return Err(FakeLakeError::CSVError(e));
            }
        };

        let mut column_names: Vec<&str> = vec![];
        for column in &config.columns {
            column_names.push(&column.name);
        }
        if let Err(e) = wtr.write_record(column_names) {
            return Err(FakeLakeError::CSVError(e));
        }

        for i in 0..rows {
            let mut row: Vec<String> = vec![];
            for column in &config.columns {
                let mut str_value = "".to_string();
                if column.is_next_present() {
                    str_value = match column.provider.value(i) {
                        Value::Int32(value) => value.to_string(),
                        Value::String(value) => value,
                        Value::Date(value) => value.to_string(),
                    };
                }
                row.push(str_value);
            }
            if let Err(e) = wtr.write_record(row) {
                return Err(FakeLakeError::CSVError(e));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Column, Config, Info};
    use crate::options::presence;
    use crate::providers::increment::integer::IncrementIntegerProvider;
    use crate::providers::random::date::date::DateProvider;
    use crate::providers::random::string::alphanumeric::AlphanumericProvider;

    use yaml_rust::YamlLoader;

    fn get_config(nb_columns: u8, name: Option<String>, rows: Option<u32>) -> Config {
        let mut columns = vec![];

        for _ in 0..nb_columns {
            columns.push(Column {
                name: "id".to_string(),
                provider: Box::new(IncrementIntegerProvider { start: 0 }),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("presence: 1").unwrap()[0],
                ),
            });
        }

        Config {
            columns,
            info: Some(Info {
                output_name: name,
                output_format: Some("csv".to_string()),
                rows,
            }),
        }
    }

    #[test]
    fn given_get_extension() {
        let output = OutputCsv {};
        assert_eq!(output.get_extension(), ".csv");
    }

    #[test]
    fn given_config_without_columns_should_error() {
        let config = get_config(0, None, None);
        let output = OutputCsv {};
        match output.generate_from_config(&config) {
            Err(_) => (),
            Ok(_) => panic!("Should fail"),
        }
    }

    #[test]
    fn given_config_without_info_should_write_file() {
        let config = get_config(1, None, None);
        let output = OutputCsv {};
        match output.generate_from_config(&config) {
            Ok(_) => (),
            Err(_) => panic!("Error"),
        }
    }

    #[test]
    fn given_config_should_write_file() {
        let config = get_config(1, Some("output_name".to_string()), Some(1000));
        let output = OutputCsv {};
        match output.generate_from_config(&config) {
            Ok(_) => (),
            Err(_) => panic!("Error"),
        }
    }

    #[test]
    fn given_all_providers_values_should_write_file() {
        let columns = vec![
            Column {
                name: "id".to_string(),
                provider: Box::new(IncrementIntegerProvider { start: 0 }),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("presence: 1").unwrap()[0],
                ),
            },
            Column {
                name: "id".to_string(),
                provider: Box::new(AlphanumericProvider {}),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("presence: 1").unwrap()[0],
                ),
            },
            Column {
                name: "id".to_string(),
                provider: Box::new(DateProvider {
                    format: "%Y-%m-%d".to_string(),
                    after: 0,
                    before: 10000,
                }),
                presence: presence::new_from_yaml(
                    &YamlLoader::load_from_str("presence: 1").unwrap()[0],
                ),
            },
        ];

        let config = Config {
            columns,
            info: Some(Info {
                output_name: Some("output_name".to_string()),
                output_format: Some("csv".to_string()),
                rows: Some(1000),
            }),
        };

        let output = OutputCsv {};
        match output.generate_from_config(&config) {
            Ok(_) => (),
            Err(_) => panic!("Error"),
        }
    }
}

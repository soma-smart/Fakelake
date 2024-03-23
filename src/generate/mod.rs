pub mod csv;
pub mod json;
pub mod output_format;
pub mod parquet;

use crate::config;
use crate::errors::FakeLakeError;
use csv::OutputCsv;
use json::OutputJson;
use output_format::OutputFormat;
use parquet::OutputParquet;

use log::{debug, info, warn};
use std::path::PathBuf;

pub fn generate_from_paths(paths_to_config: Vec<PathBuf>) -> Result<(), FakeLakeError> {
    let mut res: Result<(), FakeLakeError> = Ok(());

    for path in paths_to_config {
        debug!("Parsing YAML file at: {:?}", path);

        let file_content = match std::fs::read_to_string(&path) {
            Ok(value) => value,
            Err(value) => {
                res = Err(FakeLakeError::BadYAMLFormat(value.to_string()));
                continue;
            }
        };

        match generate_from_string(&path, file_content) {
            Ok(_) => info!("File from path {:?} generated.", &path),
            Err(e) => {
                res = Err(FakeLakeError::BadYAMLFormat(format!(
                    "Unexpected error during file generation from path {:?}: {}",
                    &path, e
                )));
            }
        };
    }

    res
}

fn generate_from_string(_: &PathBuf, file_content: String) -> Result<(), FakeLakeError> {
    let config = config::get_config_from_string(file_content)?;
    debug!("Parsed YAML config: {:?}", config);
    generate_from_config(config)
}

pub fn generate_from_config(config: config::Config) -> Result<(), FakeLakeError> {
    let output = get_corresponding_output(&config);
    output.generate_from_config(&config)
}

fn get_corresponding_output(config: &config::Config) -> Box<dyn OutputFormat> {
    match &config.info {
        Some(info) => match &info.output_format {
            Some(output_format) => match output_format {
                config::OutputType::Parquet() => Box::new(OutputParquet),
                config::OutputType::Csv(value) => Box::new(OutputCsv::new(*value)),
                config::OutputType::Json(value) => Box::new(OutputJson::new(*value)),
            },
            None => wrong_format(),
        },
        None => wrong_format(),
    }
}

fn wrong_format() -> Box<dyn OutputFormat> {
    warn!("No output format specified, the file will be in parquet.");
    Box::new(OutputParquet)
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, Info, OutputType};

    use super::*;

    fn expecting_ok<T, E>(res: &Result<T, E>) {
        match res {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    fn expecting_err<T, E>(res: &Result<T, E>) {
        match res {
            Err(_) => (),
            _ => panic!(),
        }
    }

    // get_corresponding_output
    #[test]
    fn given_no_info_should_call_parquet_generation() {
        let config = Config {
            columns: Vec::new(),
            info: None,
        };

        let output = get_corresponding_output(&config);
        assert_eq!(output.get_extension(), OutputParquet.get_extension());
    }

    #[test]
    fn given_info_empty_should_call_parquet_generation() {
        let info = Some(Info {
            output_name: None,
            output_format: None,
            rows: None,
        });
        let config = Config {
            columns: Vec::new(),
            info,
        };

        let output = get_corresponding_output(&config);
        assert_eq!(output.get_extension(), OutputParquet.get_extension());
    }

    #[test]
    fn given_parquet_format_should_call_parquet_generation() {
        let info = Some(Info {
            output_name: None,
            output_format: Some(OutputType::Parquet()),
            rows: None,
        });
        let config = Config {
            columns: Vec::new(),
            info,
        };

        let output = get_corresponding_output(&config);
        assert_eq!(output.get_extension(), OutputParquet.get_extension());
    }

    // Wrong Format
    #[test]
    fn given_nothing_should_wrong_format_call_parquet_generation() {
        let _ = Config {
            columns: Vec::new(),
            info: None,
        };

        let output = wrong_format();
        assert_eq!(output.get_extension(), OutputParquet.get_extension());
    }

    fn paths_to_vec_pathbuf(path: &str) -> Vec<PathBuf> {
        let path = PathBuf::from(path);
        vec![path]
    }

    // generate_from_paths
    #[test]
    fn given_no_files_should_return_ok() {
        let output = generate_from_paths(Vec::new());
        expecting_ok(&output);
    }

    #[test]
    fn given_not_existing_file_should_skip_and_return_err() {
        let paths = paths_to_vec_pathbuf("this/is/not/an/existing/file");
        let output = generate_from_paths(paths);
        expecting_err(&output);
    }

    #[test]
    fn given_existing_file_but_not_yaml_should_err() {
        let paths = paths_to_vec_pathbuf("src/generate/generate.rs");
        let output = generate_from_paths(paths);
        expecting_err(&output);
    }

    #[test]
    fn given_existing_file_should_return_ok() {
        let paths = paths_to_vec_pathbuf("tests/one_row_parquet.yaml");
        let output = generate_from_paths(paths);
        expecting_ok(&output);
    }
}

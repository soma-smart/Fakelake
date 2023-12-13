/// Config structs used by Fakelake during YAML parsing
///
use std::{path::PathBuf, collections::BTreeMap};

use arrow_schema::DataType;
use yaml_rust::YamlLoader;

use crate::errors::FakeLakeError;
use crate::providers::integer::auto_increment::AutoIncrementProvider;
use crate::providers::provider::Provider;
use crate::providers::string::email::EmailProvider;

#[derive(Debug)]
pub struct Config {
    pub columns: Vec<Column>,
    pub info: Option<Info>,
}

#[derive(Debug)]
pub struct Column {
    pub name: String,
    pub provider: Box<dyn Provider>,
}

#[derive(Debug)]
pub struct Info {
    /// If not specified, output_name takes the name of the input file
    pub output_name: Option<String>,
    /// By default, output_format is Parquet
    pub output_format: Option<String>,
    pub rows: Option<u32>,
}


pub fn get_config_from_path(path: &PathBuf) -> Result<Config, FakeLakeError> {
    let file_content = std::fs::read_to_string(&path)?;

    let mut columns = Vec::new();

    let parsed_yaml = match YamlLoader::load_from_str(&file_content) {
        Ok(docs) => docs,
        Err(e) => return Err(FakeLakeError::BadYAMLFormat(e.to_string())),
    };

    // iter over columns
    for column in parsed_yaml[0]["columns"].as_vec().unwrap() {
        let name = column["name"].as_str().unwrap();
        let provider = column["provider"].as_str().unwrap();

        let provider: Box<dyn Provider> = match provider {
            "auto-increment" => Box::new(AutoIncrementProvider::new_from_yaml(&column)),
            "email" => Box::new(EmailProvider::new_from_yaml(&column)),
            _ => panic!("Unknown provider: {}", provider),
        };

        let column = Column { name: name.to_string(), provider };
        columns.push(column);
    }

    // parse info section
    let section_info = &parsed_yaml[0]["info"];
    let info = Info {
        output_name: match section_info["output_name"].as_str() {
            Some(name) => Some(name.to_string()),
            None => None,
        },
        output_format: match section_info["output_format"].as_str() {
            Some(format) => Some(format.to_string()),
            None => None,
        },
        // rows could be i64 or str (i64 with _ separators)
        rows: match section_info["rows"].as_i64() {
            Some(rows) => Some(rows as u32),
            None => match section_info["rows"].as_str() {
                Some(rows) => Some(rows.replace("_", "").parse::<u32>().unwrap()),
                None => None,
            },
        },
    };

    let config = Config { columns, info: Some(info) };

    return Ok(config);
}
use log::{debug, error, info, warn};
use std::path::PathBuf;

use crate::{config, errors::FakeLakeError};
use crate::generate::{ output_format::OutputFormat, parquet_format::OutputParquet };

pub fn generate_from_paths(paths_to_config: Vec<PathBuf>) -> Result<(), FakeLakeError> {
    for path in paths_to_config {
        debug!("Parsing YAML file at: {:?}", path);
        let config = config::get_config_from_path(&path)?;
        debug!("Parsed YAML config: {:?}", config);
        match generate_from_config(config) {
            Err(e) => error!("Unexpected error during file generation from path {:?}: {}", &path, e),
            Ok(_) => info!("File from path {:?} generated.", &path),
        };
    }

    Ok(())
}

pub fn generate_from_config(config: config::Config) -> Result<(), FakeLakeError> {
    match &config.info {
        Some(info) => match &info.output_format {
            Some(output_format) => match output_format.as_str() {
                "parquet" => OutputParquet::generate_from_config(&config),
                _ => {
                    warn!("Unknown output format specified, the file will be in parquet.");
                    OutputParquet::generate_from_config(&config)
                },
            },
            None => wrong_format(config),
        },
        None => wrong_format(config),
    }
}

fn wrong_format(config: config::Config) -> Result<(), FakeLakeError> {
    warn!("No output format specified, the file will be in parquet.");
    OutputParquet::generate_from_config(&config)
}
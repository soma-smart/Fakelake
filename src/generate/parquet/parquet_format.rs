use crate::config::Config;
use crate::errors::FakeLakeError;
use crate::generate::output_format::OutputFormat;
use super::{batch_generator::{ ParquetBatchGenerator, parquet_batch_generator_builder }, utils};

use arrow_array::{ ArrayRef, Int32Array, RecordBatch };
use arrow_schema::{Field, Schema};
use log::debug;
use parquet::{arrow::ArrowWriter, basic::Compression, file::properties::WriterProperties};
use rayon::prelude::*;
use std::sync::{ Arc, Mutex };

const PARQUET_EXTENSION: &str = ".parquet";

fn get_output_file_name(config: &Config) -> &str {
    match &config.info {
        Some(info) => match &info.output_name {
            Some(name) => name,
            None => "output",
        },
        None => "output",
    }
}

fn get_number_of_rows(config: &Config) -> u32 {
    match &config.info {
        Some(info) => match info.rows {
            Some(rows) => rows,
            None => 1_000_000,
        },
        None => 1_000_000,
    }
}

#[derive(Debug, PartialEq)]
pub struct OutputParquet;

impl OutputFormat for OutputParquet {
    fn get_extension(&self) -> &str {
        return PARQUET_EXTENSION;
    }

    fn generate_from_config(&self, config: &Config) -> Result<(), FakeLakeError> {
        if config.columns.len() == 0 {
            return Err(FakeLakeError::BadYAMLFormat("No columns to generate".to_string()));
        }

        let file_name = get_output_file_name(config);
        let rows = get_number_of_rows(config);

        let schema = get_schema_from_config(config);
        debug!("Writing schema: {:?}", schema);
        
        // WriterProperties can be used to set Parquet file options
        let props = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build();

        let batch_size = 8192 * 8;
        // ceil division
        let iterations = (rows as f64 / batch_size as f64).ceil() as u32;
    
        let file = std::fs::File::create(format!("{}{}", file_name, PARQUET_EXTENSION)).unwrap();
        let mut writer = ArrowWriter::try_new(file, Arc::new(schema.clone()), Some(props)).unwrap();

        let mut schema_cols: Vec<(String, ArrayRef)> = Vec::new();
        let mut provider_generators: Vec<Box<dyn ParquetBatchGenerator>> = Vec::new();
        config.columns.clone().into_iter().for_each(|column| {
            schema_cols.push((column.clone().name, Arc::new(Int32Array::from(vec![0])) as ArrayRef));
            provider_generators.push(parquet_batch_generator_builder(column.clone()))
        });

        for i in 0..iterations {
            debug!("Generating batch {} of {}...", i, iterations);
            let rows_to_generate = if i == iterations - 1 {
                rows - (i * batch_size)
            } else {
                batch_size
            };

            let schema_cols: Mutex<Vec<(String, ArrayRef)>> = Mutex::new(schema_cols.clone());
            let provider_generators = provider_generators.clone();

            provider_generators.into_par_iter().enumerate().for_each(|(index, provider_generator)| {
                let array = provider_generator.batch_array(rows_to_generate);
                schema_cols.lock().unwrap()[index] = (provider_generator.name().to_string(), array);
            });

            let batch = RecordBatch::try_from_iter(schema_cols.lock().unwrap().clone()).unwrap();
            writer.write(&batch).expect("Writing batch");
        }
        // writer must be closed to write footer
        writer.close().unwrap();
        Ok(())
    }
}

fn get_schema_from_config(config: &Config) -> Schema {
    let mut fields = Vec::new();

    for column in &config.columns {
        let parquet_type = utils::get_parquet_type_from_column(column.clone());
        fields.push(Field::new(&column.name, parquet_type, column.can_be_null()));
    }

    Schema::new(fields)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ Column, Config, Info };
    use crate::providers::increment::integer::IncrementIntegerProvider;
    use crate::options::presence;

    use yaml_rust::YamlLoader;

    #[test]
    fn given_config_get_schema() {
        let mut columns = Vec::new();
        columns.push(Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: presence::new_from_yaml(&YamlLoader::load_from_str("presence: 1").unwrap()[0])
        });

        let config = Config {
            columns,
            info: Some(Info { output_name: None, output_format: None, rows: None })
        };
        let schema = get_schema_from_config(&config);

        assert_eq!(schema.fields().len(), 1);
        assert_eq!(schema.fields()[0].name(), "id");
    }

    #[test]
    fn given_get_extension() {
        let output_parquet = OutputParquet { };
        assert_eq!(output_parquet.get_extension(), ".parquet");
    }

    #[test]
    fn given_no_infos_default_rows() {
        let mut columns = Vec::new();
        columns.push(Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: presence::new_from_yaml(&YamlLoader::load_from_str("presence: 1").unwrap()[0])
        });
        let config = Config {
            columns,
            info: None
        };
        assert_eq!(get_number_of_rows(&config), 1_000_000);
    }

    #[test]
    fn given_no_infos_default_output_name() {
        let mut columns = Vec::new();
        columns.push(Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: presence::new_from_yaml(&YamlLoader::load_from_str("presence: 1").unwrap()[0])
        });
        let config = Config {
            columns,
            info: None
        };
        assert_eq!(get_output_file_name(&config), "output");
    }

    #[test]
    fn given_no_rows() {
        let mut columns = Vec::new();
        columns.push(Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: presence::new_from_yaml(&YamlLoader::load_from_str("presence: 1").unwrap()[0])
        });
        let config = Config {
            columns,
            info: Some(Info { output_name: None, output_format: None, rows: None })
        };
        assert_eq!(get_number_of_rows(&config), 1_000_000);
    }

    #[test]
    fn given_rows() {
        let mut columns = Vec::new();
        columns.push(Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: presence::new_from_yaml(&YamlLoader::load_from_str("presence: 1").unwrap()[0])
        });
        let config = Config {
            columns,
            info: Some(Info { output_name: None, output_format: None, rows: Some(2_466_619) })
        };
        assert_eq!(get_number_of_rows(&config), 2_466_619);
    }

    #[test]
    fn given_no_output_name() {
        let mut columns = Vec::new();
        columns.push(Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: presence::new_from_yaml(&YamlLoader::load_from_str("presence: 1").unwrap()[0])
        });
        let config = Config {
            columns,
            info: Some(Info { output_name: None, output_format: None, rows: None })
        };
        assert_eq!(get_output_file_name(&config), "output");
    }

    #[test]
    fn given_output_name() {
        let mut columns = Vec::new();
        columns.push(Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: presence::new_from_yaml(&YamlLoader::load_from_str("presence: 1").unwrap()[0])
        });
        let config = Config {
            columns,
            info: Some(Info { output_name: Some("not_default_file".to_string()), output_format: None, rows: None })
        };
        assert_eq!(get_output_file_name(&config), "not_default_file");
    }

    #[test]
    fn given_normal_config_should_generate_file() {
        let mut columns = Vec::new();
        columns.push(Column {
            name: "id".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: presence::new_from_yaml(&YamlLoader::load_from_str("presence: 1").unwrap()[0])
        });
        let config = Config {
            columns,
            info: Some(Info { output_name: Some("not_default_file".to_string()), output_format: None, rows: None })
        };

        let output_parquet = OutputParquet {};
        match output_parquet.generate_from_config(&config) {
            Ok(_) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn given_no_column_should_not_generate_file() {
        let columns = Vec::new();
        let config = Config {
            columns,
            info: Some(Info { output_name: Some("not_default_file".to_string()), output_format: None, rows: None })
        };

        let output_parquet = OutputParquet {};
        match output_parquet.generate_from_config(&config) {
            Err(_) => assert!(true),
            _ => assert!(false)
        }
    }
}
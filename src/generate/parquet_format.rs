use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray, Date32Array};
use arrow_schema::{Field, DataType, Schema};
use log::debug;
use parquet::{arrow::ArrowWriter, basic::Compression, file::properties::WriterProperties};
use std::sync::Arc;

use crate::config::Config;
use crate::errors::FakeLakeError;
use crate::generate::output_format::OutputFormat;
use crate::providers::provider;

const PARQUET_EXTENSION: &str = ".parquet";

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

        let file_name = match &config.info {
            Some(info) => match &info.output_name {
                Some(name) => name,
                None => "output",
            },
            None => "output",
        };
        
        let rows = match &config.info {
            Some(info) => match info.rows {
                Some(rows) => rows,
                None => 1_000_000,
            },
            None => 1_000_000,
        };

        let schema = get_schema_from_config(config);
        debug!("Writing schema: {:?}", schema);
        
        // WriterProperties can be used to set Parquet file options
        let props = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build();

        let batch_size = 8192;
        // ceil division
        let iterations = (rows as f64 / batch_size as f64).ceil() as u32;
    
        let file = std::fs::File::create(format!("{}{}", file_name, PARQUET_EXTENSION)).unwrap();
        let mut writer = ArrowWriter::try_new(file, Arc::new(schema), Some(props)).unwrap();
    
        for i in 0..iterations {
            debug!("Generating batch {} of {}...", i, iterations);
            let rows_to_generate = if i == iterations - 1 {
                rows - (i * batch_size)
            } else {
                batch_size
            };
    
            let mut schema_cols = Vec::new();
    
            for column in &config.columns {
    
                let parquet_type = column.provider.get_parquet_type();
    
                let array = match &parquet_type {
                    DataType::Int32 => {
                        let mut vec: Vec<Option<i32>> = Vec::new();
                        for i in 0..rows_to_generate {
                            if column.is_next_present() {
                                match column.provider.value(i) {
                                    provider::Value::Int32(value) => vec.push(Some(value)),
                                    _ => panic!("Wrong provider type"),
                                }
                            } else {
                                vec.push(None)
                            }
                        }
                        Arc::new(Int32Array::from(vec)) as ArrayRef
                    },
                    DataType::Utf8 => {
                        let mut vec: Vec<Option<String>> = Vec::new();
                        for i in 0..rows_to_generate {
                            if column.is_next_present() {
                                match column.provider.value(i) {
                                    provider::Value::String(value) => vec.push(Some(value)),
                                    _ => panic!("Wrong provider type"),
                                }
                            } else {
                                vec.push(None)
                            }
                        }
                        Arc::new(StringArray::from(vec)) as ArrayRef
                    },
                    DataType::Date32 => {
                        let mut vec: Vec<Option<i32>> = Vec::new();
                        for i in 0..rows_to_generate {
                            if column.is_next_present() {
                                match column.provider.value(i) {
                                    provider::Value::Date(value) => vec.push(Some(value)),
                                    _ => panic!("Wrong provider type"),
                                }
                            } else {
                                vec.push(None)
                            }
                        }
                        Arc::new(Date32Array::from(vec)) as ArrayRef
                    },
                    _ => panic!("Unknown parquet type: {:?}", parquet_type),
                };
                schema_cols.push((&column.name, array));
            }
            let batch = RecordBatch::try_from_iter(schema_cols).unwrap();
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
        // let parquet_type = provider::provider_conf_to_provider(&column.provider).get_parquet_type();
        let parquet_type = column.provider.get_parquet_type();
        fields.push(Field::new(&column.name, parquet_type, column.can_be_null()));
    }

    Schema::new(fields)
}
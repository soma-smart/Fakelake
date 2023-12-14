use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray};
use arrow_schema::{Field, DataType, Schema};
use log::debug;
use parquet::{arrow::ArrowWriter, basic::Compression, file::properties::WriterProperties};
use std::sync::Arc;

use crate::config::Config;
use crate::errors::FakeLakeError;
use crate::generate::output_format::OutputFormat;
use crate::providers::provider;

const PARQUET_EXTENSION: &str = ".parquet";

pub struct OutputParquet;

impl OutputFormat for OutputParquet {
    fn generate_from_config(config: &Config) -> Result<(), FakeLakeError> {
        let file_name = match &config.info {
            Some(info) => match &info.output_name {
                Some(name) => name ,
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
                            match column.provider.value(i) {
                                Some(provider::Value::Int32(value)) => vec.push(Some(value)),
                                None => vec.push(None),
                                _ => panic!("Wrong provider type"),
                            }
                        }
                        Arc::new(Int32Array::from(vec)) as ArrayRef
                    },
                    DataType::Utf8 => {
                        let mut vec: Vec<Option<String>> = Vec::new();
                        for i in 0..rows_to_generate {
                            match column.provider.value(i) {
                                Some(provider::Value::String(value)) => vec.push(Some(value)),
                                None => vec.push(None),
                                _ => panic!("Wrong provider type"),
                            }
                        }
                        Arc::new(StringArray::from(vec)) as ArrayRef
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
        fields.push(Field::new(&column.name, parquet_type, true));
    }

    Schema::new(fields)
}
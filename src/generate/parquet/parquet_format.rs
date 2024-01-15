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
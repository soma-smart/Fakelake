use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray};
use log::{debug, info};
use parquet::{arrow::ArrowWriter, basic::Compression, file::properties::WriterProperties};
use arrow_schema::{Field, DataType, Schema};
use std::path::PathBuf;
use std::sync::Arc;

use crate::{config, errors::FakeLakeError};
use crate::providers::provider;

pub fn generate_from_paths(paths_to_config: Vec<PathBuf>) -> Result<(), FakeLakeError> {
    for path in paths_to_config {
        debug!("Parsing YAML file at: {:?}", path);
        let config = config::get_config_from_path(&path)?;
        debug!("Parsed YAML config: {:?}", config);
        generate_from_config(config);
        info!("File from path {:?} generated.", &path);
    }

    Ok(())
}

pub fn generate_from_config(config: config::Config) {
    debug!("Generating data from config: {:?}...", config);

    let batch_size = 8192;
    let rows = match &config.info {
        Some(info) => match info.rows {
            Some(rows) => rows,
            None => 1_000_000,
        },
        None => 1_000_000,
    };

    let file_name = match &config.info {
        Some(info) => match &info.output_name {
            Some(name) => name,
            None => "output",
        },
        None => "output",
    };

    // ceil division
    let iterations = (rows as f64 / batch_size as f64).ceil() as u32;

    let file = std::fs::File::create(file_name).unwrap();

    // WriterProperties can be used to set Parquet file options
    let props = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    let schema = get_schema_from_config(&config);

    debug!("Writing schema: {:?}", schema);

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
}

fn get_schema_from_config(config: &config::Config) -> Schema {
    let mut fields = Vec::new();

    for column in &config.columns {
        // let parquet_type = provider::provider_conf_to_provider(&column.provider).get_parquet_type();
        let parquet_type = column.provider.get_parquet_type();
        fields.push(Field::new(&column.name, parquet_type, true));
    }

    Schema::new(fields)
}
use crate::providers::provider::Value;
use crate::config::Column;
use arrow_array::{ Array, ArrayRef, Date32Array, Int32Array, StringArray };
use std::sync::Arc;
use super::utils::get_parquet_type_from_column;
use arrow_schema::DataType;

pub trait CloneParquetBatchGenerator {
    fn clone_box(&self) -> Box<dyn ParquetBatchGenerator>;
}

impl<T> CloneParquetBatchGenerator for T
where
    T: 'static + ParquetBatchGenerator + Clone,
{
    fn clone_box(&self) -> Box<dyn ParquetBatchGenerator> {
        Box::new(self.clone())
    }
}


pub trait ParquetBatchGenerator: CloneParquetBatchGenerator + Send + Sync {
    fn batch_array(&self, rows_to_generate: u32) -> Arc<dyn Array>;
    fn name(&self) -> &str;
    fn new(column: Column) -> Self where Self: Sized;
}

impl Clone for Box<dyn ParquetBatchGenerator> {
    fn clone(&self) -> Box<dyn ParquetBatchGenerator> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct IntBatchGenerator {
    column: Column
}
impl ParquetBatchGenerator for IntBatchGenerator {
    fn batch_array(&self, rows_to_generate: u32) -> Arc<dyn Array> {
        let mut vec: Vec<Option<i32>> = Vec::new();
        for i in 0..rows_to_generate {
            if self.column.is_next_present() {
                match self.column.provider.value(i) {
                    Value::Int32(value) => vec.push(Some(value)),
                    _ => panic!("Wrong provider type"),
                }
            } else {
                vec.push(None)
            }
        }
        Arc::new(Int32Array::from(vec)) as ArrayRef
    }

    fn name(&self) -> &str {
        &self.column.name
    }

    fn new(column: Column) -> IntBatchGenerator {
        IntBatchGenerator {
            column
        }
    }
}


#[derive(Clone)]
struct StrBatchGenerator {
    column: Column
}
impl ParquetBatchGenerator for StrBatchGenerator {
    fn batch_array(&self, rows_to_generate: u32) -> Arc<dyn Array> {
        let mut vec: Vec<Option<String>> = Vec::new();
        for i in 0..rows_to_generate {
            if self.column.is_next_present() {
                match self.column.provider.value(i) {
                    Value::String(value) => vec.push(Some(value)),
                    _ => panic!("Wrong provider type"),
                }
            } else {
                vec.push(None)
            }
        }
        Arc::new(StringArray::from(vec)) as ArrayRef
    }

    fn name(&self) -> &str {
        &self.column.name
    }

    fn new(column: Column) -> StrBatchGenerator {
        StrBatchGenerator {
            column
        }
    }
}


#[derive(Clone)]
struct DateBatchGenerator {
    column: Column
}
impl ParquetBatchGenerator for DateBatchGenerator {
    fn batch_array(&self, rows_to_generate: u32) -> Arc<dyn Array> {
        let mut vec: Vec<Option<i32>> = Vec::new();
        for i in 0..rows_to_generate {
            if self.column.is_next_present() {
                match self.column.provider.value(i) {
                    Value::Date(value) => vec.push(Some(value)),
                    _ => panic!("Wrong provider type"),
                }
            } else {
                vec.push(None)
            }
        }
        Arc::new(Date32Array::from(vec)) as ArrayRef
    }

    fn name(&self) -> &str {
        &self.column.name
    }

    fn new(column: Column) -> DateBatchGenerator {
        DateBatchGenerator {
            column
        }
    }
}

pub fn parquet_batch_generator_builder(column: Column) -> Box<dyn ParquetBatchGenerator>{
    match get_parquet_type_from_column(column.clone()) {
        DataType::Int32 => Box::new(IntBatchGenerator::new(column.clone())),
        DataType::Utf8 => Box::new(StrBatchGenerator::new(column.clone())),
        DataType::Date32 => Box::new(DateBatchGenerator::new(column.clone())),
        _ => panic!("Parquet type expected not handled.")
    }
}
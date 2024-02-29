use super::utils::get_parquet_type_from_column;
use crate::config::Column;
use crate::providers::provider::Value;
use arrow_array::{Array, ArrayRef, BooleanArray, Date32Array, Int32Array, StringArray};
use arrow_schema::DataType;
use chrono::{Datelike, NaiveDate};
use std::sync::Arc;

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
    fn new(column: Column) -> Self
    where
        Self: Sized;
}

impl Clone for Box<dyn ParquetBatchGenerator> {
    fn clone(&self) -> Box<dyn ParquetBatchGenerator> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct BoolBatchGenerator {
    column: Column,
}
impl ParquetBatchGenerator for BoolBatchGenerator {
    fn batch_array(&self, rows_to_generate: u32) -> Arc<dyn Array> {
        let mut vec: Vec<Option<bool>> = Vec::new();
        for i in 0..rows_to_generate {
            if self.column.is_next_present() {
                match self.column.provider.value(i) {
                    Value::Bool(value) => vec.push(Some(value)),
                    _ => panic!("Wrong provider type"),
                }
            } else {
                vec.push(None)
            }
        }
        Arc::new(BooleanArray::from(vec)) as ArrayRef
    }
    fn name(&self) -> &str {
        &self.column.name
    }
    fn new(column: Column) -> BoolBatchGenerator {
        BoolBatchGenerator { column }
    }
}

#[derive(Clone)]
struct IntBatchGenerator {
    column: Column,
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
        IntBatchGenerator { column }
    }
}

#[derive(Clone)]
struct StrBatchGenerator {
    column: Column,
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
        StrBatchGenerator { column }
    }
}

#[derive(Clone)]
struct DateBatchGenerator {
    column: Column,
}
impl ParquetBatchGenerator for DateBatchGenerator {
    fn batch_array(&self, rows_to_generate: u32) -> Arc<dyn Array> {
        let epoch = NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d").unwrap();

        let mut vec: Vec<Option<i32>> = Vec::new();
        for i in 0..rows_to_generate {
            if self.column.is_next_present() {
                match self.column.provider.value(i) {
                    Value::Date(value) => {
                        vec.push(Some(value.num_days_from_ce() - epoch.num_days_from_ce()))
                    }
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
        DateBatchGenerator { column }
    }
}

pub fn parquet_batch_generator_builder(column: Column) -> Box<dyn ParquetBatchGenerator> {
    match get_parquet_type_from_column(column.clone()) {
        DataType::Boolean => Box::new(BoolBatchGenerator::new(column.clone())),
        DataType::Int32 => Box::new(IntBatchGenerator::new(column.clone())),
        DataType::Utf8 => Box::new(StrBatchGenerator::new(column.clone())),
        DataType::Date32 => Box::new(DateBatchGenerator::new(column.clone())),
        _ => panic!("Parquet type expected not handled."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::presence::new_from_yaml;
    use crate::providers::{
        increment::integer::IncrementIntegerProvider, random::bool::BoolProvider,
        random::date::date::DateProvider, random::string::alphanumeric::AlphanumericProvider,
    };

    use yaml_rust::YamlLoader;

    // Bool batch generator
    #[test]
    fn given_bool_provider_should_return_batch_generator() {
        let column = Column {
            name: "bool_column".to_string(),
            provider: Box::new(BoolProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };

        let ret = parquet_batch_generator_builder(column);
        assert_eq!(ret.name(), "bool_column");
    }

    #[test]
    fn given_bool_batch_generator_should_batch_correctly() {
        let column = Column {
            name: "bool_column".to_string(),
            provider: Box::new(BoolProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };
        let batch_generator = BoolBatchGenerator { column };
        let arr = batch_generator.batch_array(1000);

        assert_eq!(arr.len(), 1000);
    }

    #[test]
    fn given_bool_batch_generator_with_presence_should_batch_correctly() {
        let column = Column {
            name: "bool_column".to_string(),
            provider: Box::new(BoolProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("presence: 0.5").unwrap()[0]),
        };
        let batch_generator = BoolBatchGenerator { column };
        let arr = batch_generator.batch_array(1000);

        assert_eq!(arr.len(), 1000);
    }

    #[test]
    #[should_panic]
    fn given_bool_batch_generator_with_wrong_provider_should_panic() {
        let column = Column {
            name: "bool_column".to_string(),
            provider: Box::new(AlphanumericProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: temp").unwrap()[0]),
        };
        let batch_generator = BoolBatchGenerator { column };
        let _ = batch_generator.batch_array(1);
    }

    // Int32 batch generator
    #[test]
    fn given_int_provider_should_return_batch_generator() {
        let column = Column {
            name: "int_column".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };

        let ret = parquet_batch_generator_builder(column);
        assert_eq!(ret.name(), "int_column");
    }

    #[test]
    fn given_int_batch_generator_should_batch_correctly() {
        let column = Column {
            name: "int_column".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };
        let batch_generator = IntBatchGenerator { column };
        let arr = batch_generator.batch_array(1000);

        assert_eq!(arr.len(), 1000);
    }

    #[test]
    fn given_int_batch_generator_with_presence_should_batch_correctly() {
        let column = Column {
            name: "int_column".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: new_from_yaml(&YamlLoader::load_from_str("presence: 0.5").unwrap()[0]),
        };
        let batch_generator = IntBatchGenerator { column };
        let arr = batch_generator.batch_array(1000);

        assert_eq!(arr.len(), 1000);
    }

    #[test]
    #[should_panic]
    fn given_int_batch_generator_with_wrong_provider_should_panic() {
        let column = Column {
            name: "int_column".to_string(),
            provider: Box::new(AlphanumericProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: temp").unwrap()[0]),
        };
        let batch_generator = IntBatchGenerator { column };
        let _ = batch_generator.batch_array(1);
    }

    // String batch generator
    #[test]
    fn given_str_provider_should_return_batch_generator() {
        let column = Column {
            name: "str_column".to_string(),
            provider: Box::new(AlphanumericProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };

        let ret = parquet_batch_generator_builder(column);
        assert_eq!(ret.name(), "str_column");
    }

    #[test]
    fn given_str_batch_generator_should_batch_correctly() {
        let column = Column {
            name: "str_column".to_string(),
            provider: Box::new(AlphanumericProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };
        let batch_generator = StrBatchGenerator { column };
        let arr = batch_generator.batch_array(1000);

        assert_eq!(arr.len(), 1000);
    }

    #[test]
    fn given_str_batch_generator_with_presence_should_batch_correctly() {
        let column = Column {
            name: "str_column".to_string(),
            provider: Box::new(AlphanumericProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("presence: 0.5").unwrap()[0]),
        };
        let batch_generator = StrBatchGenerator { column };
        let arr = batch_generator.batch_array(1000);

        assert_eq!(arr.len(), 1000);
    }

    #[test]
    #[should_panic]
    fn given_str_batch_generator_with_wrong_provider_should_panic() {
        let column = Column {
            name: "str_column".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: temp").unwrap()[0]),
        };
        let batch_generator = StrBatchGenerator { column };
        let _ = batch_generator.batch_array(1);
    }

    // Date batch generator
    #[test]
    fn given_date_provider_should_return_batch_generator() {
        let column = Column {
            name: "date_column".to_string(),
            provider: Box::new(DateProvider {
                format: "%Y-%m-%d".to_string(),
                before: 100,
                after: 0,
            }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };

        let ret = parquet_batch_generator_builder(column);
        assert_eq!(ret.name(), "date_column");
    }

    #[test]
    fn given_date_batch_generator_should_batch_correctly() {
        let column = Column {
            name: "date_column".to_string(),
            provider: Box::new(DateProvider {
                format: "%Y-%m-%d".to_string(),
                before: 100,
                after: 0,
            }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };
        let batch_generator = DateBatchGenerator { column };
        let arr = batch_generator.batch_array(1000);

        assert_eq!(arr.len(), 1000);
    }

    #[test]
    fn given_date_batch_generator_with_presence_should_batch_correctly() {
        let column = Column {
            name: "date_column".to_string(),
            provider: Box::new(DateProvider {
                format: "%Y-%m-%d".to_string(),
                before: 100,
                after: 0,
            }),
            presence: new_from_yaml(&YamlLoader::load_from_str("presence: 0.5").unwrap()[0]),
        };
        let batch_generator = DateBatchGenerator { column };
        let arr = batch_generator.batch_array(1000);

        assert_eq!(arr.len(), 1000);
    }

    #[test]
    #[should_panic]
    fn given_date_batch_generator_with_wrong_provider_should_panic() {
        let column = Column {
            name: "date_column".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: temp").unwrap()[0]),
        };
        let batch_generator = DateBatchGenerator { column };
        let _ = batch_generator.batch_array(1);
    }
}

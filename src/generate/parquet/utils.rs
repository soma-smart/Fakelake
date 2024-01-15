use arrow_schema::DataType;
use crate::config::Column;
use crate::providers::provider::Value;

pub fn get_parquet_type_from_column(column: Column) -> DataType {
    match column.provider.value(0) {
        Value::Int32(_) => DataType::Int32,
        Value::String(_) => DataType::Utf8,
        Value::Date(_) => DataType::Date32
    }
}
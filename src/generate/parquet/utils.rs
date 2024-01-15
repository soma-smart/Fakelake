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

#[cfg(test)]
mod tests {
    use super::get_parquet_type_from_column;

    use crate::config::Column;
    use crate::providers::{
        increment::integer::IncrementIntegerProvider,
        random::string::alphanumeric::AlphanumericProvider,
        random::date::date::DateProvider
    };
    use crate::options::presence::new_from_yaml;

    use arrow_schema::DataType;
    use yaml_rust::YamlLoader;

    #[test]
    fn given_int_provider_should_return_int_datatype() {
        let column = Column {
            name: "int_column".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0 }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0])
        };
        assert_eq!(get_parquet_type_from_column(column), DataType::Int32);
    }

    #[test]
    fn given_str_provider_should_return_utf8_datatype() {
        let column = Column {
            name: "str_column".to_string(),
            provider: Box::new(AlphanumericProvider { }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0])
        };
        assert_eq!(get_parquet_type_from_column(column), DataType::Utf8);
    }

    #[test]
    fn given_date_provider_should_return_date_datatype() {
        let column = Column {
            name: "date_column".to_string(),
            provider: Box::new(DateProvider { format: "%Y-%m-%d".to_string(), before: 100, after: 0 }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0])
        };
        assert_eq!(get_parquet_type_from_column(column), DataType::Date32);
    }
}
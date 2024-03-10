use crate::config::Column;
use crate::providers::provider::Value;
use arrow_schema::{DataType, TimeUnit};

pub fn get_parquet_type_from_column(column: Column) -> DataType {
    match column.provider.value(0) {
        Value::Bool(_) => DataType::Boolean,
        Value::Int32(_) => DataType::Int32,
        Value::String(_) => DataType::Utf8,
        Value::Date(_, _) => DataType::Date32,
        Value::Timestamp(_, _) => DataType::Timestamp(TimeUnit::Second, None),
    }
}

#[cfg(test)]
mod tests {
    use super::get_parquet_type_from_column;

    use crate::config::Column;
    use crate::options::presence::new_from_yaml;
    use crate::providers::{
        increment::integer::IncrementIntegerProvider, random::bool::BoolProvider,
        random::date::date::DateProvider, random::date::datetime::DatetimeProvider,
        random::string::alphanumeric::AlphanumericProvider,
    };

    use arrow_schema::{DataType, TimeUnit};
    use yaml_rust::YamlLoader;

    #[test]
    fn given_bool_provider_should_return_bool_datatype() {
        let column = Column {
            name: "bool_column".to_string(),
            provider: Box::new(BoolProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };
        assert_eq!(get_parquet_type_from_column(column), DataType::Boolean);
    }

    #[test]
    fn given_int_provider_should_return_int_datatype() {
        let column = Column {
            name: "int_column".to_string(),
            provider: Box::new(IncrementIntegerProvider { start: 0, step: 1 }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };
        assert_eq!(get_parquet_type_from_column(column), DataType::Int32);
    }

    #[test]
    fn given_str_provider_should_return_utf8_datatype() {
        let column = Column {
            name: "str_column".to_string(),
            provider: Box::new(AlphanumericProvider {}),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };
        assert_eq!(get_parquet_type_from_column(column), DataType::Utf8);
    }

    #[test]
    fn given_date_provider_should_return_date_datatype() {
        let column = Column {
            name: "date_column".to_string(),
            provider: Box::new(DateProvider {
                format: "%Y-%m-%d".to_string(),
                before: 100,
                after: 0,
            }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };
        assert_eq!(get_parquet_type_from_column(column), DataType::Date32);
    }

    #[test]
    fn given_timestamp_provider_should_return_timestamp_datatype() {
        let column = Column {
            name: "timestamp_column".to_string(),
            provider: Box::new(DatetimeProvider {
                format: "%Y-%m-%d %H:%M:%S".to_string(),
                after: 10_000_000,
                before: 12_000_000,
            }),
            presence: new_from_yaml(&YamlLoader::load_from_str("name: test").unwrap()[0]),
        };
        assert_eq!(
            get_parquet_type_from_column(column),
            DataType::Timestamp(TimeUnit::Second, None)
        );
    }
}

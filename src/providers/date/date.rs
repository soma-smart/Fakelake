use arrow_schema::DataType;
use yaml_rust::Yaml;
use chrono::{NaiveDate, Datelike};
use log::{ info, warn };

use crate::providers::provider::{Provider, Value};
use crate::providers::column_options::ColumnOptions;

const DEFAULT_FORMAT: &str = "%Y-%m-%d";
const DEFAULT_AFTER: &str = "1980-01-01";
const DEFAULT_BEFORE: &str = "2000-01-01";

pub struct DateProvider {
    pub options: Option<ColumnOptions>,
    pub format: String,
    pub after: i32,
    pub before: i32,
}

impl Provider for DateProvider {
    fn value(&self, index: u32) -> Option<Value> {
        let calculated_value = Value::Date(fastrand::i32(self.after..self.before));
        return match &self.options {
            Some(value) => value.alter_value(calculated_value, index),
            _ => Some(calculated_value),
        }
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Date32;
    }
    fn new_from_yaml(column: &Yaml) -> DateProvider {
        let column_options = ColumnOptions::new_from_yaml(column);

        let format_option = match column["format"].as_str() {
            Some(format) => format,
            None => {
                warn!("No format parameter, default will be {}", DEFAULT_FORMAT);
                DEFAULT_FORMAT
            },
        };

        let after_parameter = match column["after"].as_str() {
            Some(after) => after,
            None => {
                warn!("No after parameter, default will be {}", DEFAULT_AFTER);
                DEFAULT_AFTER
            },
        };
        let before_parameter = match column["before"].as_str() {
            Some(after) => after,
            None => {
                warn!("No before parameter, default will be {}", DEFAULT_BEFORE);
                DEFAULT_BEFORE
            },
        };
        let check_after_format = NaiveDate::parse_from_str(after_parameter, format_option);
        let check_before_format = NaiveDate::parse_from_str(before_parameter, format_option);

        let after_option: NaiveDate;
        let before_option: NaiveDate;
        match (check_after_format, check_before_format) {
            (Ok(after), Ok(before)) => {
                if before < after {
                    info!("After parameter should be the low interval but it is higher than the Before parameter. Both values are switched for the rest of the generation.");
                    after_option = before;
                    before_option = after;
                } else {
                    after_option = after;
                    before_option = before;
                }
            },
            // If format can't be applied to after or before, put everything to default value
            (_, _) => {
                warn!("Error while applying the format ({}) to after ({}) and before ({}) parameters. Default value used.", format_option, after_parameter, before_parameter);
                after_option = match NaiveDate::parse_from_str(DEFAULT_AFTER, DEFAULT_FORMAT) {
                    Ok(after) => after,
                    _ => panic!("Issue with default format and default after date parsing.")
                };
                before_option = match NaiveDate::parse_from_str(DEFAULT_BEFORE, DEFAULT_FORMAT) {
                    Ok(before) => before,
                    _ => panic!("Issue with default format and default before date parsing.")
                };
            }
        }

        let epoch = match NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d") {
            Ok(epoch) => epoch,
            Err(e) => panic!("Issue with epoch calculation: {}", e)
        };
        let after_option_in_days = after_option.num_days_from_ce() - epoch.num_days_from_ce();
        let before_option_in_days = before_option.num_days_from_ce() - epoch.num_days_from_ce();

        return DateProvider {
            options: column_options,
            format: format_option.to_string(),
            after: after_option_in_days,
            before: before_option_in_days
        };
    }
}

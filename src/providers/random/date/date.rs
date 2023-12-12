use crate::providers::provider::{Provider, Value};

use chrono::{Datelike, NaiveDate};
use log::{info, warn};
use yaml_rust::Yaml;

const DEFAULT_FORMAT: &str = "%Y-%m-%d";
const DEFAULT_AFTER: &str = "1980-01-01";
const DEFAULT_BEFORE: &str = "2000-01-01";

#[derive(Clone)]
pub struct DateProvider {
    pub format: String,
    pub after: i32,
    pub before: i32,
}

impl Provider for DateProvider {
    fn value(&self, _: u32) -> Value {
        Value::Date(fastrand::i32(self.after..self.before))
    }
    fn new_from_yaml(column: &Yaml) -> DateProvider {
        let mut format_option = match column["format"].as_str() {
            Some(format) => format,
            None => {
                warn!("No format parameter, default will be {}", DEFAULT_FORMAT);
                DEFAULT_FORMAT
            }
        };

        let after_parameter = match column["after"].as_str() {
            Some(after) => after,
            None => {
                warn!("No after parameter, default will be {}", DEFAULT_AFTER);
                DEFAULT_AFTER
            }
        };
        let before_parameter = match column["before"].as_str() {
            Some(after) => after,
            None => {
                warn!("No before parameter, default will be {}", DEFAULT_BEFORE);
                DEFAULT_BEFORE
            }
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
            }
            // If format can't be applied to after or before, put everything to default value
            (_, _) => {
                warn!("Error while applying the format ({}) to after ({}) and before ({}) parameters. Default value used.", format_option, after_parameter, before_parameter);
                format_option = DEFAULT_FORMAT;
                after_option = NaiveDate::parse_from_str(DEFAULT_AFTER, DEFAULT_FORMAT).unwrap();
                before_option = NaiveDate::parse_from_str(DEFAULT_BEFORE, DEFAULT_FORMAT).unwrap();
            }
        }

        let epoch = NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d").unwrap();
        let after_option_in_days = after_option.num_days_from_ce() - epoch.num_days_from_ce();
        let before_option_in_days = before_option.num_days_from_ce() - epoch.num_days_from_ce();

        DateProvider {
            format: format_option.to_string(),
            after: after_option_in_days,
            before: before_option_in_days,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DateProvider, DEFAULT_AFTER, DEFAULT_BEFORE, DEFAULT_FORMAT};
    use crate::providers::provider::{Provider, Value};

    use chrono::{Datelike, NaiveDate};
    use yaml_rust::YamlLoader;

    fn generate_provider(
        format: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> DateProvider {
        let yaml_format = match format {
            Some(value) => format!("{}format: \"{}\"", "\n", value),
            None => String::new(),
        };
        let yaml_after = match after {
            Some(value) => format!("{}after: {}", "\n", value),
            None => String::new(),
        };
        let yaml_before = match before {
            Some(value) => format!("{}before: {}", "\n", value),
            None => String::new(),
        };

        let yaml_str = format!("name: id{}{}{}", yaml_format, yaml_after, yaml_before);

        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        DateProvider::new_from_yaml(&yaml[0])
    }

    fn get_epoch() -> NaiveDate {
        match NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d") {
            Ok(value) => value,
            Err(_) => panic!("Should not happen as it is a tested environment"),
        }
    }

    fn get_day_since_epoch(date: &str, format: &str) -> i32 {
        match NaiveDate::parse_from_str(date, format) {
            Ok(value) => value.num_days_from_ce() - get_epoch().num_days_from_ce(),
            Err(_) => panic!("Should not happen as it is a tested environment"),
        }
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider: DateProvider = generate_provider(None, None, None);
        match provider.value(0) {
            Value::Date(_) => (),
            _ => panic!(),
        };
    }

    // Validate YAML file
    #[test]
    fn given_no_params_should_give_default() {
        let provider: DateProvider = generate_provider(None, None, None);

        assert_eq!(provider.format, DEFAULT_FORMAT);
        assert_eq!(
            provider.before,
            get_day_since_epoch(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            provider.after,
            get_day_since_epoch(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_no_format_should_give_default_format() {
        let after = "2000-01-14";
        let before = "2020-01-14";
        let provider: DateProvider = generate_provider(None, Some(after), Some(before));

        assert_eq!(provider.format, DEFAULT_FORMAT);
        assert_eq!(provider.before, get_day_since_epoch(before, DEFAULT_FORMAT));
        assert_eq!(provider.after, get_day_since_epoch(after, DEFAULT_FORMAT));
    }

    #[test]
    fn given_no_before_should_give_default_before() {
        let format = "%Y-%m-%d";
        let after = "1980-01-14";
        let provider: DateProvider = generate_provider(Some(format), Some(after), None);

        assert_eq!(provider.format, format);
        assert_eq!(provider.before, get_day_since_epoch(DEFAULT_BEFORE, format));
        assert_eq!(provider.after, get_day_since_epoch(after, format));
    }

    #[test]
    fn given_no_after_should_give_default_after() {
        let format = "%Y-%m-%d";
        let before = "2000-01-14";
        let provider: DateProvider = generate_provider(Some(format), None, Some(before));

        assert_eq!(provider.format, format);
        assert_eq!(provider.before, get_day_since_epoch(before, format));
        assert_eq!(provider.after, get_day_since_epoch(DEFAULT_AFTER, format));
    }

    #[test]
    fn given_wrong_after_format_should_give_default() {
        let format = "%d-%m-%Y";
        let after = "2000-01-17";
        let before = "14-01-2020";
        let provider: DateProvider = generate_provider(Some(format), Some(after), Some(before));

        assert_eq!(provider.format, DEFAULT_FORMAT);
        assert_eq!(
            provider.before,
            get_day_since_epoch(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            provider.after,
            get_day_since_epoch(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_wrong_before_format_should_give_default() {
        let format = "%d-%m-%Y";
        let after = "17-01-2000";
        let before = "2020-01-14";
        let provider: DateProvider = generate_provider(Some(format), Some(after), Some(before));

        assert_eq!(provider.format, DEFAULT_FORMAT);
        assert_eq!(
            provider.before,
            get_day_since_epoch(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            provider.after,
            get_day_since_epoch(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_after_and_before_in_wrong_order_should_switch_them() {
        let format = "%d-%m-%Y";
        let after = "17-01-2020";
        let before = "14-01-2000";
        let provider: DateProvider = generate_provider(Some(format), Some(after), Some(before));

        assert_eq!(provider.format, format);
        assert_eq!(provider.before, get_day_since_epoch(after, format));
        assert_eq!(provider.after, get_day_since_epoch(before, format));
    }

    // Validate value calculation
    #[test]
    fn given_provider_should_return_between_after_inclusive_and_before_exclusive() {
        let provider = DateProvider {
            format: DEFAULT_FORMAT.to_string(),
            after: get_day_since_epoch(DEFAULT_AFTER, DEFAULT_FORMAT),
            before: get_day_since_epoch(DEFAULT_BEFORE, DEFAULT_FORMAT),
        };

        for value in 1..100 {
            match provider.value(value) {
                Value::Date(value) => {
                    assert!(value >= provider.after);
                    assert!(value < provider.before);
                }
                _ => panic!("Wrong type"),
            }
        }
    }

    #[test]
    fn given_one_day_range_should_return_always_same_date() {
        let provider = DateProvider {
            format: DEFAULT_FORMAT.to_string(),
            after: get_day_since_epoch("2020-05-18", DEFAULT_FORMAT),
            before: get_day_since_epoch("2020-05-19", DEFAULT_FORMAT),
        };

        for value in 1..100 {
            match provider.value(value) {
                Value::Date(value) => {
                    assert_eq!(value, get_day_since_epoch("2020-05-18", DEFAULT_FORMAT));
                }
                _ => panic!("Wrong type"),
            }
        }
    }
}

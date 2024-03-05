use crate::providers::provider::{Provider, Value};

use chrono::NaiveDateTime;
use log::{info, warn};
use yaml_rust::Yaml;

const DEFAULT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const DEFAULT_AFTER: &str = "1980-01-01 12:00:00";
const DEFAULT_BEFORE: &str = "2000-01-01 12:00:00";

#[derive(Clone)]
pub struct DatetimeProvider {
    pub format: String,
    pub after: i64,
    pub before: i64,
}

impl Provider for DatetimeProvider {
    fn value(&self, _: u32) -> Value {
        Value::Timestamp(
            NaiveDateTime::from_timestamp_opt(fastrand::i64(self.after..self.before), 0).unwrap(),
            self.format.clone(),
        )
    }
    fn new_from_yaml(column: &Yaml) -> DatetimeProvider {
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
        let check_after_format = NaiveDateTime::parse_from_str(after_parameter, format_option);
        let check_before_format = NaiveDateTime::parse_from_str(before_parameter, format_option);

        let after_option: NaiveDateTime;
        let before_option: NaiveDateTime;
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
                after_option =
                    NaiveDateTime::parse_from_str(DEFAULT_AFTER, DEFAULT_FORMAT).unwrap();
                before_option =
                    NaiveDateTime::parse_from_str(DEFAULT_BEFORE, DEFAULT_FORMAT).unwrap();
            }
        }

        let after_option_in_seconds = after_option.timestamp();
        let before_option_in_seconds = before_option.timestamp();

        DatetimeProvider {
            format: format_option.to_string(),
            after: after_option_in_seconds,
            before: before_option_in_seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DatetimeProvider, DEFAULT_AFTER, DEFAULT_BEFORE, DEFAULT_FORMAT};
    use crate::providers::provider::{Provider, Value};

    use chrono::NaiveDateTime;
    use yaml_rust::YamlLoader;

    fn generate_provider(
        format: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
    ) -> DatetimeProvider {
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
        DatetimeProvider::new_from_yaml(&yaml[0])
    }

    fn get_seconds_since_day0(date: &str, format: &str) -> i64 {
        match NaiveDateTime::parse_from_str(date, format) {
            Ok(value) => value.timestamp(),
            Err(_) => panic!("Should not happen as it is a tested environment"),
        }
    }

    #[test]
    fn given_nothing_should_return_timestamp_type() {
        let provider = generate_provider(None, None, None);
        match provider.value(0) {
            Value::Timestamp(_, _) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn given_no_params_should_give_default_timestamp() {
        let provider = generate_provider(None, None, None);

        assert_eq!(provider.format, DEFAULT_FORMAT);
        assert_eq!(
            provider.before,
            get_seconds_since_day0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            provider.after,
            get_seconds_since_day0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_no_format_should_give_default_format() {
        let after = "2000-01-14 12:00:00";
        let before = "2020-01-14 12:00:00";
        let provider: DatetimeProvider = generate_provider(None, Some(after), Some(before));

        assert_eq!(provider.format, DEFAULT_FORMAT);
        assert_eq!(
            provider.before,
            get_seconds_since_day0(before, DEFAULT_FORMAT)
        );
        assert_eq!(
            provider.after,
            get_seconds_since_day0(after, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_no_before_should_give_default_before() {
        let format = "%Y-%m-%d %H:%M:%S";
        let after = "1980-01-14 12:00:00";
        let provider: DatetimeProvider = generate_provider(Some(format), Some(after), None);

        assert_eq!(provider.format, format);
        assert_eq!(
            provider.before,
            get_seconds_since_day0(DEFAULT_BEFORE, format)
        );
        assert_eq!(provider.after, get_seconds_since_day0(after, format));
    }

    #[test]
    fn given_no_after_should_give_default_after() {
        let format = "%Y-%m-%d %H:%M:%S";
        let before = "2000-01-14 12:00:00";
        let provider: DatetimeProvider = generate_provider(Some(format), None, Some(before));

        assert_eq!(provider.format, format);
        assert_eq!(provider.before, get_seconds_since_day0(before, format));
        assert_eq!(
            provider.after,
            get_seconds_since_day0(DEFAULT_AFTER, format)
        );
    }

    #[test]
    fn given_wrong_after_format_should_give_default() {
        let format = "%d-%m-%Y %M:%H:%S";
        let after = "2000-01-17 12:40:00";
        let before = "14-01-2020 00:12:00";
        let provider: DatetimeProvider = generate_provider(Some(format), Some(after), Some(before));

        assert_eq!(provider.format, DEFAULT_FORMAT);
        assert_eq!(
            provider.before,
            get_seconds_since_day0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            provider.after,
            get_seconds_since_day0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_wrong_before_format_should_give_default() {
        let format = "%d-%m-%Y %M:%H:%S";
        let after = "17-01-2000 00:12:00";
        let before = "2020-01-14 12:40:00";
        let provider: DatetimeProvider = generate_provider(Some(format), Some(after), Some(before));

        assert_eq!(provider.format, DEFAULT_FORMAT);
        assert_eq!(
            provider.before,
            get_seconds_since_day0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            provider.after,
            get_seconds_since_day0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_after_and_before_in_wrong_order_should_switch_them() {
        let format = "%d-%m-%Y %M:%H:%S";
        let after = "17-01-2020 40:12:00";
        let before = "14-01-2000 40:12:00";
        let provider: DatetimeProvider = generate_provider(Some(format), Some(after), Some(before));

        assert_eq!(provider.format, format);
        assert_eq!(provider.before, get_seconds_since_day0(after, format));
        assert_eq!(provider.after, get_seconds_since_day0(before, format));
    }

    // Validate value calculation
    #[test]
    fn given_provider_should_return_between_after_inclusive_and_before_exclusive() {
        let provider = DatetimeProvider {
            format: DEFAULT_FORMAT.to_string(),
            after: get_seconds_since_day0(DEFAULT_AFTER, DEFAULT_FORMAT),
            before: get_seconds_since_day0(DEFAULT_BEFORE, DEFAULT_FORMAT),
        };

        for value in 1..100 {
            match provider.value(value) {
                Value::Timestamp(value, _) => {
                    assert!(value.timestamp() >= provider.after);
                    assert!(value.timestamp() < provider.before);
                }
                _ => panic!("Wrong type"),
            }
        }
    }

    #[test]
    fn given_one_second_range_should_return_always_same_date() {
        let provider = DatetimeProvider {
            format: DEFAULT_FORMAT.to_string(),
            after: get_seconds_since_day0("2020-05-18 12:00:00", DEFAULT_FORMAT),
            before: get_seconds_since_day0("2020-05-18 12:00:01", DEFAULT_FORMAT),
        };

        for value in 1..100 {
            match provider.value(value) {
                Value::Timestamp(value, _) => {
                    assert_eq!(
                        value.timestamp(),
                        get_seconds_since_day0("2020-05-18 12:00:00", DEFAULT_FORMAT)
                    );
                }
                _ => panic!("Wrong type"),
            }
        }
    }
}

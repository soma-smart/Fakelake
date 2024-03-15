use crate::providers::parameters::datetime::DatetimeParameter;
use crate::providers::provider::{Provider, Value};

use chrono::NaiveDateTime;
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
        let date_time_parameter =
            DatetimeParameter::new(column, DEFAULT_FORMAT, DEFAULT_AFTER, DEFAULT_BEFORE);

        DatetimeProvider {
            format: date_time_parameter.format,
            after: date_time_parameter.after,
            before: date_time_parameter.before,
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
    fn given_every_params_should_give_same_timestamps() {
        let format = "%d-%m-%Y %H:%M:%S";
        let after = "17-01-2000 12:00:00";
        let before = "17-01-2020 12:00:00";
        let provider = generate_provider(Some(format), Some(after), Some(before));

        assert_eq!(provider.format, format);
        assert_eq!(provider.before, get_seconds_since_day0(before, format));
        assert_eq!(provider.after, get_seconds_since_day0(after, format));
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

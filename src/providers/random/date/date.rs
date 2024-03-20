use crate::providers::parameters::date::DateParameter;
use crate::providers::provider::{Provider, Value};

use chrono::{Datelike, NaiveDate};
use yaml_rust::Yaml;

const DEFAULT_FORMAT: &str = "%Y-%m-%d";
const DEFAULT_AFTER: &str = "1980-01-01";
const DEFAULT_BEFORE: &str = "2000-01-01";

const MIN_DATE: NaiveDate = NaiveDate::MIN;
const MAX_DATE: NaiveDate = NaiveDate::MAX;

#[derive(Clone)]
pub struct DateProvider {
    pub format: String,
    pub after: i32,
    pub before: i32,
}

impl Provider for DateProvider {
    fn value(&self, _: u32) -> Value {
        Value::Date(
            NaiveDate::from_num_days_from_ce_opt(fastrand::i32(self.after..self.before)).unwrap(),
            self.format.clone(),
        )
    }
    fn corrupted_value(&self, _: u32) -> Value {
        Value::Date(
            NaiveDate::from_num_days_from_ce_opt(fastrand::i32(
                MIN_DATE.num_days_from_ce()..MAX_DATE.num_days_from_ce(),
            ))
            .unwrap(),
            self.format.clone(),
        )
    }
}

pub fn new_from_yaml(column: &Yaml) -> Box<DateProvider> {
    let parameter = DateParameter::new(column, DEFAULT_FORMAT, DEFAULT_AFTER, DEFAULT_BEFORE);

    Box::new(DateProvider {
        format: parameter.format,
        after: parameter.after,
        before: parameter.before,
    })
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
    ) -> Box<DateProvider> {
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
        super::new_from_yaml(&yaml[0])
    }

    fn get_day_since_year0(date: &str, format: &str) -> i32 {
        match NaiveDate::parse_from_str(date, format) {
            Ok(value) => value.num_days_from_ce(),
            Err(_) => panic!("Should not happen as it is a tested environment"),
        }
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider = generate_provider(None, None, None);
        match provider.value(0) {
            Value::Date(_, _) => (),
            _ => panic!(),
        };
    }

    // Validate YAML file
    #[test]
    fn given_no_params_should_give_default() {
        let provider = generate_provider(None, None, None);

        assert_eq!(provider.format, DEFAULT_FORMAT);
        assert_eq!(
            provider.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            provider.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_every_params_should_give_same_timestamps() {
        let format = "%d-%m-%Y";
        let after = "17-01-2000";
        let before = "17-01-2020";
        let provider = generate_provider(Some(format), Some(after), Some(before));

        assert_eq!(provider.format, format);
        assert_eq!(provider.before, get_day_since_year0(before, format));
        assert_eq!(provider.after, get_day_since_year0(after, format));
    }

    // Validate value calculation
    #[test]
    fn given_provider_should_return_between_after_inclusive_and_before_exclusive() {
        let provider = DateProvider {
            format: DEFAULT_FORMAT.to_string(),
            after: get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT),
            before: get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT),
        };

        for value in 1..100 {
            match provider.value(value) {
                Value::Date(value, _) => {
                    assert!(value.num_days_from_ce() >= provider.after);
                    assert!(value.num_days_from_ce() < provider.before);
                }
                _ => panic!("Wrong type"),
            }
        }
    }

    #[test]
    fn given_one_day_range_should_return_always_same_date() {
        let provider = DateProvider {
            format: DEFAULT_FORMAT.to_string(),
            after: get_day_since_year0("2020-05-18", DEFAULT_FORMAT),
            before: get_day_since_year0("2020-05-19", DEFAULT_FORMAT),
        };

        for value in 1..100 {
            match provider.value(value) {
                Value::Date(value, _) => {
                    assert_eq!(
                        value.num_days_from_ce(),
                        get_day_since_year0("2020-05-18", DEFAULT_FORMAT)
                    );
                }
                _ => panic!("Wrong type"),
            }
        }
    }

    #[test]
    fn given_provider_should_corrupted_return_random_date() {
        let provider = DateProvider {
            format: DEFAULT_FORMAT.to_string(),
            after: get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT),
            before: get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT),
        };

        let mut count_random_date = 0;
        for value in 1..100 {
            match provider.corrupted_value(value) {
                Value::Date(value, _) => {
                    if value.num_days_from_ce() < provider.after
                        || value.num_days_from_ce() > provider.before
                    {
                        count_random_date += 1
                    }
                }
                _ => panic!("Wrong type"),
            }
        }
        assert!(count_random_date >= 99)
    }
}

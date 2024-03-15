use chrono::{Datelike, NaiveDate};
use yaml_rust::Yaml;

use log::warn;

use super::get_column_name;

pub struct DateParameter {
    pub format: String,
    pub after: i32,
    pub before: i32,
}

fn extract_format(column: &Yaml, default_format: &str) -> Result<String, ()> {
    match &column["format"] {
        Yaml::String(value) => Ok(value.clone()),
        Yaml::BadValue => Ok(default_format.to_owned()),
        _ => Err(()),
    }
}

fn extract_datetime(
    column: &Yaml,
    column_name: &str,
    default_value: &str,
    format: &str,
) -> Result<i32, ()> {
    match &column[column_name] {
        Yaml::String(value) => str_datetime_to_i64(format, value),
        Yaml::BadValue => str_datetime_to_i64(format, default_value),
        _ => Err(()),
    }
}

fn str_datetime_to_i64(format: &str, str_datetime: &str) -> Result<i32, ()> {
    match NaiveDate::parse_from_str(str_datetime, format) {
        Ok(value) => Ok(value.num_days_from_ce()),
        Err(_) => Err(()),
    }
}

impl DateParameter {
    pub fn new(
        column: &Yaml,
        default_format: &str,
        default_after: &str,
        default_before: &str,
    ) -> DateParameter {
        let column_name = get_column_name(column);

        let format_option;
        match extract_format(column, default_format) {
            Ok(value) => {
                format_option = value;
            }
            Err(_) => {
                return wrap_up_issue(column_name, default_format, default_after, default_before)
            }
        }

        let after_parameter;
        match extract_datetime(column, "after", default_after, &format_option) {
            Ok(value) => {
                after_parameter = value;
            }
            Err(_) => {
                return wrap_up_issue(column_name, default_format, default_after, default_before)
            }
        }

        let before_parameter;
        match extract_datetime(column, "before", default_before, &format_option) {
            Ok(value) => {
                before_parameter = value;
            }
            Err(_) => {
                return wrap_up_issue(column_name, default_format, default_after, default_before)
            }
        }

        if before_parameter < after_parameter {
            return wrap_up_issue(column_name, default_format, default_after, default_before);
        }

        DateParameter {
            format: format_option.to_string(),
            after: after_parameter,
            before: before_parameter,
        }
    }
}

fn wrap_up_issue(column_name: &str, format: &str, after: &str, before: &str) -> DateParameter {
    warn!(
        "Column {} after/before should be the same format as the param, with after > before.{} Value {}, {} - {} taken instead.",
        column_name, "\n", format, after, before
    );
    DateParameter {
        format: format.to_string(),
        after: str_datetime_to_i64(format, after).unwrap(),
        before: str_datetime_to_i64(format, before).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDate};
    use yaml_rust::YamlLoader;

    use super::DateParameter;

    const DEFAULT_FORMAT: &str = "%Y-%m-%d";
    const DEFAULT_AFTER: &str = "1980-01-01";
    const DEFAULT_BEFORE: &str = "2000-01-01";

    fn generate_yaml(
        format_value: Option<&str>,
        after_value: Option<&str>,
        before_value: Option<&str>,
    ) -> DateParameter {
        let yaml_format = match format_value {
            Some(value) => format!("format: \"{}\"{}", value, "\n"),
            None => String::new(),
        };

        let yaml_after = match after_value {
            Some(value) => format!("after: {}{}", value, "\n"),
            None => String::new(),
        };

        let yaml_before = match before_value {
            Some(value) => format!("before: {}{}", value, "\n"),
            None => String::new(),
        };

        let yaml_str = format!(
            "name: datetime_col{}{}{}{}",
            "\n", yaml_format, yaml_after, yaml_before
        );
        let column = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();

        DateParameter::new(&column[0], DEFAULT_FORMAT, DEFAULT_AFTER, DEFAULT_BEFORE)
    }

    fn get_day_since_year0(date: &str, format: &str) -> i32 {
        match NaiveDate::parse_from_str(date, format) {
            Ok(value) => value.num_days_from_ce(),
            Err(_) => panic!("Should not happen as it is a tested environment"),
        }
    }

    #[test]
    fn given_bad_format_should_return_default() {
        let format = "-3";
        let after = "2000-01-14";
        let before = "2020-01-14";
        let parameter = generate_yaml(Some(format), Some(after), Some(before));

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            parameter.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_bad_after_should_return_default() {
        let format = "%Y-%m-%d";
        let after = "-3";
        let before = "2020-01-14";
        let parameter = generate_yaml(Some(format), Some(after), Some(before));

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            parameter.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_bad_before_should_return_default() {
        let format = "%Y-%m-%d";
        let after = "2020-01-14";
        let before = "-3";
        let parameter = generate_yaml(Some(format), Some(after), Some(before));

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            parameter.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_no_params_should_give_default_timestamp() {
        let parameter = generate_yaml(None, None, None);

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            parameter.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_no_format_should_give_default_format_when_after_before_match() {
        let after = "2000-01-14";
        let before = "2020-01-14";
        let parameter = generate_yaml(None, Some(after), Some(before));

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(before, DEFAULT_FORMAT)
        );
        assert_eq!(parameter.after, get_day_since_year0(after, DEFAULT_FORMAT));
    }

    #[test]
    fn given_no_format_should_give_default_all_when_after_dont_match() {
        let after = "14-01-2000";
        let before = "2020-01-14";
        let parameter = generate_yaml(None, Some(after), Some(before));

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            parameter.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_no_format_should_give_default_all_when_before_dont_match() {
        let after = "2000-01-14";
        let before = "14-01-2020";
        let parameter = generate_yaml(None, Some(after), Some(before));

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            parameter.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_no_after_should_give_default_after() {
        let format = "%Y-%m-%d";
        let before = "2000-01-14";
        let parameter = generate_yaml(Some(format), None, Some(before));

        assert_eq!(parameter.format, format);
        assert_eq!(parameter.before, get_day_since_year0(before, format));
        assert_eq!(parameter.after, get_day_since_year0(DEFAULT_AFTER, format));
    }

    #[test]
    fn given_no_before_should_give_default_before() {
        let format = "%Y-%m-%d";
        let after = "1980-01-14";
        let parameter = generate_yaml(Some(format), Some(after), None);

        assert_eq!(parameter.format, format);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, format)
        );
        assert_eq!(parameter.after, get_day_since_year0(after, format));
    }

    #[test]
    fn given_wrong_after_format_should_give_default() {
        let format = "%d-%m-%Y";
        let after = "2000-01-17";
        let before = "14-01-2020";
        let parameter = generate_yaml(Some(format), Some(after), Some(before));

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            parameter.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_wrong_before_format_should_give_default() {
        let format = "%d-%m-%Y";
        let after = "17-01-2000";
        let before = "2020-01-14";
        let parameter = generate_yaml(Some(format), Some(after), Some(before));

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            parameter.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }

    #[test]
    fn given_after_greater_than_before_should_give_default() {
        let format = "%d-%m-%Y";
        let after = "10-07-2007";
        let before = "22-02-1997";
        let parameter = generate_yaml(Some(format), Some(after), Some(before));

        assert_eq!(parameter.format, DEFAULT_FORMAT);
        assert_eq!(
            parameter.before,
            get_day_since_year0(DEFAULT_BEFORE, DEFAULT_FORMAT)
        );
        assert_eq!(
            parameter.after,
            get_day_since_year0(DEFAULT_AFTER, DEFAULT_FORMAT)
        );
    }
}

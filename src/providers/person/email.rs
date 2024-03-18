use crate::providers::parameters::string::StringParameter;
use crate::providers::provider::{Provider, Value};
use crate::providers::utils::string::random_characters;

use yaml_rust::Yaml;

const DEFAULT_DOMAIN: &str = "example.com";

#[derive(Clone)]
pub struct EmailProvider {
    pub domain: String,
}

impl Provider for EmailProvider {
    fn value(&self, _: u32) -> Value {
        // return a random email address
        // generate a random string of length 10 (subject) + @ + random domain
        let subject: String = random_characters(10);
        Value::String(format!("{}@{}", subject, self.domain))
    }
    fn new_from_yaml(column: &Yaml) -> EmailProvider {
        let domain_parameter = StringParameter::new(column, "domain", DEFAULT_DOMAIN);

        EmailProvider {
            domain: domain_parameter.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EmailProvider, DEFAULT_DOMAIN};
    use crate::providers::provider::{Provider, Value};

    use regex::Regex;
    use yaml_rust::YamlLoader;

    fn generate_provider(start: Option<String>) -> EmailProvider {
        let yaml_str = match start {
            Some(value) => format!("name: id{}domain: {}", "\n", value),
            None => "name: id".to_string(),
        };
        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        EmailProvider::new_from_yaml(&yaml[0])
    }

    // Parquet type
    #[test]
    fn given_nothing_should_return_parquet_type() {
        let provider: EmailProvider = generate_provider(None);
        match provider.value(0) {
            Value::String(_) => (),
            _ => panic!(),
        };
    }

    // Validate YAML file
    #[test]
    fn given_no_domain_in_yaml_should_give_domain_default() {
        let provider: EmailProvider = generate_provider(None);
        assert_eq!(provider.domain, DEFAULT_DOMAIN);
    }

    #[test]
    fn given_x_for_domain_in_yaml_should_give_domain_x() {
        let values_to_check = ["test.com", "domain.org", "test.this"];
        for value in values_to_check {
            let provider = generate_provider(Some(value.to_string()));
            assert_eq!(provider.domain, value);
        }
    }

    // Validate value calculation
    #[test]
    fn given_domain_x_should_return_string_length_10_plus_1_plus_length_x() {
        let domain_to_check = ["test.com", "domain.org", "test.this"];
        for domain in domain_to_check {
            let provider = EmailProvider {
                domain: domain.to_string(),
            };
            let expected_length = 10 + "@".len() + domain.len();

            let values_to_check = [0, 4, 50];
            for value in values_to_check {
                match provider.value(value) {
                    Value::String(value) => assert_eq!(value.len(), expected_length),
                    _ => panic!("Wrong type"),
                }
            }
        }
    }

    #[test]
    fn given_domain_x_should_return_correct_email() {
        let domain_to_check = ["test.com", "domain.org", "other.this"];
        for domain in domain_to_check {
            let provider = EmailProvider {
                domain: domain.to_string(),
            };

            let pattern = format!(r"@{}$", regex::escape(domain));
            let re = Regex::new(&pattern).unwrap();

            let values_to_check = [0, 4, 50];
            for value in values_to_check {
                match provider.value(value) {
                    Value::String(value) => assert!(re.is_match(&value)),
                    _ => panic!("Wrong type"),
                }
            }
        }
    }
}

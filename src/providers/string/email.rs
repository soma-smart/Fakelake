use arrow_schema::DataType;
use yaml_rust::Yaml;

use crate::providers::provider::{Provider, Value};
use super::utils::random_characters;

const DEFAULT_DOMAIN: &str = "example.com";

pub struct EmailProvider {
    pub domain: String,
}

impl Provider for EmailProvider {
    fn value(&self, _: u32) -> Value {
        // return a random email address
        // generate a random string of length 10 (subject) + @ + random domain
        let subject: String = random_characters(10);
        return Value::String(format!("{}@{}", subject, self.domain));
    }
    fn get_parquet_type(&self) -> DataType {
        return DataType::Utf8;
    }
    fn new_from_yaml(column: &Yaml) -> EmailProvider {
        let domain_option = column["domain"].as_str().unwrap_or(DEFAULT_DOMAIN).to_string();

        return EmailProvider {
            domain: domain_option
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::providers::provider::{ Value, Provider };
    use super::EmailProvider;

    use yaml_rust::YamlLoader;
    use regex::Regex;

    fn generate_provider(start: Option<String>) -> EmailProvider {
        let yaml_str = match start {
            Some(value) => format!("name: id{}domain: {}", "\n", value),
            None => format!("name: id"),
        };
        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        EmailProvider::new_from_yaml(&yaml[0])
    }

    // Validate YAML file
    #[test]
    fn given_no_domain_in_yaml_should_give_domain_examplecom() {
        let provider: EmailProvider = generate_provider(None);
        assert_eq!(provider.domain, "example.com");
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
            let provider = EmailProvider { domain: format!("{}", domain)};
            let expected_length = 10 + "@".len() + domain.len();

            let values_to_check = [0, 4, 50];
            for value in values_to_check {
                match provider.value(value) {
                    Value::String(value) => assert_eq!(value.len(), expected_length),
                    _ => panic!("Wrong type")
                }
            }
        }
    }

    #[test]
    fn given_domain_x_should_return_correct_email() {
        let domain_to_check = ["test.com", "domain.org", "other.this"];
        for domain in domain_to_check {
            let provider = EmailProvider { domain: format!("{}", domain)};

            let pattern = format!(r"@{}$", regex::escape(domain));
            let re = Regex::new(&pattern).unwrap();

            let values_to_check = [0, 4, 50];
            for value in values_to_check {
                match provider.value(value) {
                    Value::String(value) => assert!(re.is_match(&value)),
                    _ => panic!("Wrong type")
                }
            }
        }
    }
}
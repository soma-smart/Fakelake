use crate::errors::FakeLakeError;
use crate::providers;
use crate::providers::parameters::percentage::PercentageParameter;

use chrono::{NaiveDate, NaiveDateTime};
use core::fmt;
use yaml_rust::Yaml;

#[derive(Clone, PartialEq, fmt::Debug)]
pub enum Value {
    Bool(bool),
    Int32(i32),
    Float64(f64),
    String(String),
    Date(NaiveDate, String),
    Timestamp(NaiveDateTime, String),
}

pub trait CloneProvider {
    fn clone_box(&self) -> Box<dyn Provider>;
}

impl<T> CloneProvider for T
where
    T: 'static + Provider + Clone,
{
    fn clone_box(&self) -> Box<dyn Provider> {
        Box::new(self.clone())
    }
}

pub trait Provider: CloneProvider + Send + Sync {
    fn value(&self, index: u32) -> Value;
    fn corrupted_value(&self, index: u32) -> Value;
}

pub struct CorruptedProvider {
    pub provider: Box<dyn Provider>,
    pub corrupted: f64,
}

impl Clone for CorruptedProvider {
    fn clone(&self) -> Self {
        CorruptedProvider {
            provider: self.provider.clone_box(),
            corrupted: self.corrupted,
        }
    }
}

impl Provider for CorruptedProvider {
    fn value(&self, index: u32) -> Value {
        let rnd: f64 = fastrand::f64();
        match rnd < self.corrupted {
            true => self.corrupted_value(index),
            false => self.provider.value(index),
        }
    }
    fn corrupted_value(&self, index: u32) -> Value {
        self.provider.corrupted_value(index)
    }
}

impl CorruptedProvider {
    pub fn new_from_yaml(column: &Yaml, provider: Box<dyn Provider>) -> Box<dyn Provider> {
        let corrupted = PercentageParameter::new(column, "corrupted", 0 as f64).value;

        match corrupted {
            x if x == 0 as f64 => provider,
            _ => Box::new(CorruptedProvider {
                provider,
                corrupted,
            }),
        }
    }
}

// Implement Debug for all types that implement Provider
#[cfg(not(tarpaulin_include))]
impl fmt::Debug for dyn Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Provider {{ }}")
    }
}

pub struct ProviderBuilder {}

impl ProviderBuilder {
    pub fn get_corresponding_provider(
        provider: &str,
        column: &Yaml,
    ) -> Result<Box<dyn Provider>, FakeLakeError> {
        let lowercased = provider.to_lowercase();
        let mut provider_split = lowercased.split('.');

        match provider_split.next() {
            Some("increment") => {
                providers::increment::builder::get_corresponding_provider(provider_split, column)
            }
            Some("person") => {
                providers::person::builder::get_corresponding_provider(provider_split, column)
            }
            Some("random") => {
                providers::random::builder::get_corresponding_provider(provider_split, column)
            }
            _ => Err(unknown_provider(provider)),
        }
    }
}

pub fn unknown_provider(wrong_provider: &str) -> FakeLakeError {
    FakeLakeError::BadYAMLFormat(format!("Unknown provider: {}", wrong_provider))
}

#[cfg(test)]
mod tests {
    use super::{CorruptedProvider, Provider, ProviderBuilder, Value};

    use yaml_rust::YamlLoader;

    use mockall::predicate::*;
    use mockall::*;

    #[derive(Clone)]
    struct TestProvider;
    mock! {
        pub TestProvider {}

        impl Clone for TestProvider {
            fn clone(&self) -> Self;
        }

        impl Provider for TestProvider {
            fn value(&self, index: u32) -> Value;
            fn corrupted_value(&self, index: u32) -> Value;
        }
    }

    #[test]
    fn given_increment_should_return_provider() {
        let provider_name = "increment.integer";
        let yaml_str = format!("name: test_col{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        match ProviderBuilder::get_corresponding_provider(provider_name, column) {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_person_should_return_provider() {
        let provider_name = "person.email";
        let yaml_str = format!("name: test_col{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        match ProviderBuilder::get_corresponding_provider(provider_name, column) {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_random_should_return_provider() {
        let provider_name = "random.string.alphanumeric";
        let yaml_str = format!("name: test_col{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        match ProviderBuilder::get_corresponding_provider(provider_name, column) {
            Ok(_) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn given_wrong_provider_should_return_error() {
        let provider_name = "not_a_provider";
        let yaml_str = format!("name: test_col{}provider: {}", '\n', provider_name);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        match ProviderBuilder::get_corresponding_provider(provider_name, column) {
            Err(_) => (),
            _ => panic!(),
        }
    }

    // Corrupted tests
    #[test]
    fn given_corrupted_provider_corrupted_should_clone_be_identical() {
        let mut mock_provider = MockTestProvider::new();
        let mut mock_cloned = MockTestProvider::new();

        mock_provider
            .expect_value()
            .return_const(Value::String("This is a unique string".to_string()));
        mock_cloned
            .expect_value()
            .return_const(Value::String("This is a unique string".to_string()));

        mock_provider
            .expect_corrupted_value()
            .return_const(Value::String("This is a second unique string".to_string()));
        mock_cloned
            .expect_corrupted_value()
            .return_const(Value::String("This is a second unique string".to_string()));

        mock_provider
            .expect_clone()
            .return_once(move || mock_cloned);

        let corrupted = CorruptedProvider {
            provider: Box::new(mock_provider),
            corrupted: 1.0,
        };
        let cloned_corrupted = corrupted.clone();

        let l = [1, 10, 100, 200];
        for i in l {
            assert_eq!(corrupted.value(i), cloned_corrupted.corrupted_value(i));
            assert_eq!(cloned_corrupted.value(i), corrupted.corrupted_value(i));
        }
    }

    #[test]
    fn given_no_corrupted_should_not_call_corrupt() {
        let yaml_str = "name: test_col".to_string();
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let mut mock_provider = Box::new(MockTestProvider::new());
        mock_provider.expect_corrupted_value().never();

        mock_provider
            .expect_value()
            .times(100)
            .return_const(Value::String(String::new()));

        let corr = CorruptedProvider::new_from_yaml(column, mock_provider);
        for i in 0..100 {
            corr.value(i);
        }
    }

    #[test]
    fn given_0_corrupted_should_not_call_corrupt() {
        let corrupted_value = "0";
        let yaml_str = format!("name: test_col{}corrupted: {}", "\n", corrupted_value);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let mut mock_provider = Box::new(MockTestProvider::new());
        mock_provider.expect_corrupted_value().never();

        mock_provider
            .expect_value()
            .times(100)
            .return_const(Value::String(String::new()));

        let corr = CorruptedProvider::new_from_yaml(column, mock_provider);
        for i in 0..100 {
            corr.value(i);
        }
    }

    #[test]
    fn given_1_corrupted_should_only_call_corrupt() {
        let corrupted_value = "1";
        let yaml_str = format!("name: test_col{}corrupted: {}", "\n", corrupted_value);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let mut mock_provider = Box::new(MockTestProvider::new());
        mock_provider.expect_value().never();

        mock_provider
            .expect_corrupted_value()
            .times(100)
            .return_const(Value::String(String::new()));

        let corr = CorruptedProvider::new_from_yaml(column, mock_provider);
        for i in 0..100 {
            corr.value(i);
        }
    }

    #[test]
    fn given_0_5_corrupted_should_call_corrupt() {
        let corrupted_value = "0.5";
        let yaml_str = format!("name: test_col{}corrupted: {}", "\n", corrupted_value);
        let column = &YamlLoader::load_from_str(yaml_str.as_str()).unwrap()[0];

        let mut mock_provider = Box::new(MockTestProvider::new());
        mock_provider
            .expect_corrupted_value()
            .times(450..550)
            .return_const(Value::String(String::new()));

        mock_provider
            .expect_value()
            .times(450..550)
            .return_const(Value::String(String::new()));

        let corr = CorruptedProvider::new_from_yaml(column, mock_provider);
        for i in 0..1000 {
            corr.value(i);
        }
    }
}

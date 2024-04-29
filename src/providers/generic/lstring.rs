use crate::providers::{
    provider::{Provider, Value},
    utils::string::random_alphanumeric,
};

#[derive(Clone)]
pub struct ListStringProvider {
    data: Vec<String>,
}

impl ListStringProvider {
    pub fn new(values: Vec<String>) -> Self {
        ListStringProvider {
            data: values.clone(),
        }
    }
}

impl Provider for ListStringProvider {
    fn value(&self, _: u32) -> Value {
        let index = fastrand::usize(..self.data.len());
        Value::String(self.data[index].to_string())
    }

    fn corrupted_value(&self, _: u32) -> Value {
        Value::String(random_alphanumeric(10))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn given_array_config_should_return_array_value() {
        let expected_input: Vec<String> = vec![
            "my_data".to_string(),
            "example".to_string(),
            "array".to_string(),
        ];
        let provider = ListStringProvider::new(expected_input.clone());
        for i in 0..10 {
            match provider.value(i) {
                Value::String(s) => assert!(expected_input.contains(&s)),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn given_array_config_should_return_corrupted_value() {
        let expected_input: Vec<String> = vec![
            "my_data".to_string(),
            "example".to_string(),
            "array".to_string(),
        ];
        let provider = ListStringProvider::new(expected_input.clone());
        for i in 0..10 {
            match provider.corrupted_value(i) {
                Value::String(s) => assert!(!expected_input.contains(&s)),
                _ => panic!(),
            }
        }
    }
}

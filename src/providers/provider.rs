use core::fmt;

use arrow_schema::DataType;
use yaml_rust::Yaml;

pub enum Value {
    Int32(i32),
    String(String),
    Date(i32),
}

pub trait Provider {
    fn value(&self, index: u32) -> Option<Value>;
    fn get_parquet_type(&self) -> DataType;
    fn new_from_yaml(column: &Yaml) -> Self where Self: Sized;
}

// Implement Debug for all types that implement Provider
impl fmt::Debug for dyn Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Provider {{ }}")
    }
}
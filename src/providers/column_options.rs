
use yaml_rust::Yaml;

use log::{info, warn};

use crate::providers::provider::Value;


pub struct ColumnOptions {
    pub presence: f64,
}

impl ColumnOptions {
    pub fn alter_value(&self, calculated_value: Value, _: u32) -> Option<Value> {
        if self.presence == 0.0 {
            return None
        }

        let rnd: f64 = fastrand::f64();
        if rnd > self.presence {
            return None;
        }

        return Some(calculated_value);
    }

    pub fn new_from_yaml(column: &Yaml) -> Option<ColumnOptions> {
        let presence_option = column["presence"].as_f64();

        return match presence_option {
            Some(value) if value < 0.0 => {
                warn!("Presence is set to Never because {} is below 0", value);
                Some(ColumnOptions { presence: 0.0 })
            },
            Some(value) if value > 1.0 => {
                warn!("Presence is set to Always because {} is above 1", value);
                None
            },
            Some(value) if value == 1.0 => {
                info!("Presence set to {} is the same as no presence parameter", value);
                None
            }
            Some(value) => Some(ColumnOptions{ presence: value }),
            None => None,
        };
    }
}

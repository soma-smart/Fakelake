use core::fmt;
use yaml_rust::Yaml;

use log::{info, warn};

pub trait PresenceTrait {
    fn is_next_present(&self) -> bool;
    fn can_be_null(&self) -> bool;
}

// Implement Debug for all types that implement Provider
impl fmt::Debug for dyn PresenceTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Provider {{ }}")
    }
}

pub struct SometimesPresent {
    pub presence: f64,
}

impl PresenceTrait for SometimesPresent {
    fn is_next_present(&self) -> bool {
        let rnd: f64 = fastrand::f64();
        if rnd > self.presence {
            return false;
        }

        return true
    }
    fn can_be_null(&self) -> bool {
        return true
    }
}

pub struct AlwaysPresent;
impl PresenceTrait for AlwaysPresent {
    fn is_next_present(&self) -> bool {
        true
    }
    fn can_be_null(&self) -> bool {
        return false
    }
}

pub struct NeverPresent;
impl PresenceTrait for NeverPresent {
    fn is_next_present(&self) -> bool {
        false
    }
    fn can_be_null(&self) -> bool {
        return true
    }
}

pub fn new_from_yaml(column: &Yaml) -> Box<dyn PresenceTrait> {
    let presence_option = column["presence"].as_f64().or_else(|| column["presence"].as_i64().map(|i| i as f64));

    return match presence_option {
        Some(value) if value < 0.0 => {
            warn!("Presence is set to Never because {} is below 0", value);
            Box::new(NeverPresent{})
        },
        Some(value) if value > 1.0 => {
            warn!("Presence is set to Always because {} is above 1", value);
            Box::new(AlwaysPresent{})
        },
        Some(value) if value == 0.0 => {
            info!("Column will contain only nulls.");
            Box::new(NeverPresent{})
        }
        Some(value) if value == 1.0 => {
            info!("Presence set to {} is the same as no presence parameter", value);
            Box::new(AlwaysPresent{})
        }
        Some(value) => Box::new(SometimesPresent{ presence: value }),
        None => Box::new(AlwaysPresent{}),
    };
}

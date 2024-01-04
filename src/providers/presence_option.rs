use core::fmt;
use yaml_rust::Yaml;

use log::{info, warn};

pub trait PresenceTrait {
    fn is_next_present(&self) -> bool;
    fn can_be_null(&self) -> bool;
}

// Implement Debug for all types that implement PresenceTrait
#[cfg(not(tarpaulin_include))]
impl fmt::Debug for dyn PresenceTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PresenceTrait")
    }
}

struct SometimesPresent {
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

struct AlwaysPresent;
impl PresenceTrait for AlwaysPresent {
    fn is_next_present(&self) -> bool {
        true
    }
    fn can_be_null(&self) -> bool {
        return false
    }
}

struct NeverPresent;
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

#[cfg(test)]
mod tests {
    use super::*;

    use yaml_rust::YamlLoader;

    fn generate_presence(presence: Option<&str>) -> Box<dyn PresenceTrait> {
        let yaml_str = match presence {
            Some(value) => format!("name: id{}presence: {}", "\n", value),
            None => format!("name: id"),
        };
        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        new_from_yaml(&yaml[0])
    }

    fn check_always_present(presence: Box<dyn PresenceTrait>) -> bool {
        !presence.can_be_null()
    }

    fn check_never_present(presence: Box<dyn PresenceTrait>) -> bool {
        if !presence.can_be_null() {
            return false
        }

        for _ in 0..10 {
            if presence.is_next_present() {
                return false
            }
        }
        return true
    }

    fn check_sometimes_present(presence: Box<dyn PresenceTrait>) -> bool {
        if !presence.can_be_null() {
            return false
        }

        let first_run = presence.is_next_present();
        for _ in 0..100 {
            if presence.is_next_present() != first_run {
                return true
            }
        }
        return false
    }

    // Validate YAML file
    #[test]
    fn given_no_presence_should_give_always_present() {
        let presence = generate_presence(None);
        assert!(check_always_present(presence));
    }

    #[test]
    fn given_0_int_presence_should_give_never_present() {
        let presence = generate_presence(Some("0"));
        assert!(check_never_present(presence));
    }
    #[test]
    fn given_0_float_presence_should_give_never_present() {
        let presence = generate_presence(Some("0.0"));
        assert!(check_never_present(presence));
    }

    #[test]
    fn given_less_than_0_int_presence_should_give_never_present() {
        let presence = generate_presence(Some("-5"));
        assert!(check_never_present(presence));
    }
    #[test]
    fn given_less_than_0_float_presence_should_give_never_present() {
        let presence = generate_presence(Some("-5.2"));
        assert!(check_never_present(presence));
    }

    #[test]
    fn given_bad_value_presence_should_give_always_present() {
        let presence = generate_presence(Some("BadValue"));
        assert!(check_always_present(presence));
    }

    #[test]
    fn given_1_int_presence_should_give_always_present() {
        let presence = generate_presence(Some("1"));
        assert!(check_always_present(presence));
    }
    #[test]
    fn given_1_float_presence_should_give_always_present() {
        let presence = generate_presence(Some("1.0"));
        assert!(check_always_present(presence));
    }

    #[test]
    fn given_more_than_1_int_presence_should_give_always_present() {
        let presence = generate_presence(Some("12"));
        assert!(check_always_present(presence));
    }
    #[test]
    fn given_more_than_1_float_presence_should_give_always_present() {
        let presence = generate_presence(Some("12.6"));
        assert!(check_always_present(presence));
    }
    
    #[test]
    fn given_between_0_and_1_presence_should_give_sometimes_present() {
        let presence = generate_presence(Some("0.3"));
        assert!(check_sometimes_present(presence));
    }

    // Validate presence option
    #[test]
    fn given_always_should_return_true() {
        let presence = AlwaysPresent;
        assert!(presence.can_be_null() == false);
        for _ in 1..100 {
            assert!(presence.is_next_present());
        }
    }

    #[test]
    fn given_never_should_return_false() {
        let presence = NeverPresent;
        assert!(presence.can_be_null());
        for _ in 1..100 {
            assert!(presence.is_next_present() == false);
        }
    }

    #[test]
    fn given_sometimes_should_return_random() {
        let presence = SometimesPresent { presence: 0.5 };
        assert!(presence.can_be_null());

        let mut count = 0;
        for _ in 1..10000 {
            match presence.is_next_present() {
                false => count = count - 1,
                true => count = count + 1,
            };
        }
        assert!(count < 500);
    }
}
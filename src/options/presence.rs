use core::fmt;
use yaml_rust::Yaml;

use crate::providers::parameters::percentage::PercentageParameter;

pub trait ClonePresence {
    fn clone_box(&self) -> Box<dyn Presence>;
}

impl<T> ClonePresence for T
where
    T: 'static + Presence + Clone,
{
    fn clone_box(&self) -> Box<dyn Presence> {
        Box::new(self.clone())
    }
}

pub trait Presence: ClonePresence + Send + Sync {
    fn is_next_present(&self) -> bool;
    fn can_be_null(&self) -> bool;
}

// Implement Debug for all types that implement Presence
#[cfg(not(tarpaulin_include))]
impl fmt::Debug for dyn Presence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Presence")
    }
}

#[derive(Clone)]
struct SometimesPresent {
    pub presence: f64,
}

impl Presence for SometimesPresent {
    fn is_next_present(&self) -> bool {
        let rnd: f64 = fastrand::f64();
        if rnd > self.presence {
            return false;
        }

        true
    }
    fn can_be_null(&self) -> bool {
        true
    }
}

#[derive(Clone)]
struct AlwaysPresent;
impl Presence for AlwaysPresent {
    fn is_next_present(&self) -> bool {
        true
    }
    fn can_be_null(&self) -> bool {
        false
    }
}

#[derive(Clone)]
struct NeverPresent;
impl Presence for NeverPresent {
    fn is_next_present(&self) -> bool {
        false
    }
    fn can_be_null(&self) -> bool {
        true
    }
}

pub fn new_from_yaml(column: &Yaml) -> Box<dyn Presence> {
    let parameter = PercentageParameter::new(column, "presence", 1.0);

    match parameter.value {
        value if value == 0.0 => Box::new(NeverPresent {}),
        value if value == 1.0 => Box::new(AlwaysPresent {}),
        value => Box::new(SometimesPresent { presence: value }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use yaml_rust::YamlLoader;

    fn generate_presence(presence: Option<&str>) -> Box<dyn Presence> {
        let yaml_str = match presence {
            Some(value) => format!("name: id{}presence: {}", "\n", value),
            None => "name: id".to_string(),
        };
        let yaml = YamlLoader::load_from_str(yaml_str.as_str()).unwrap();
        new_from_yaml(&yaml[0])
    }

    fn check_always_present(presence: Box<dyn Presence>) -> bool {
        !presence.can_be_null()
    }

    fn check_never_present(presence: Box<dyn Presence>) -> bool {
        if !presence.can_be_null() {
            return false;
        }

        for _ in 0..10 {
            if presence.is_next_present() {
                return false;
            }
        }
        true
    }

    fn check_sometimes_present(presence: Box<dyn Presence>) -> bool {
        if !presence.can_be_null() {
            return false;
        }

        let first_run = presence.is_next_present();
        for _ in 0..100 {
            if presence.is_next_present() != first_run {
                return true;
            }
        }
        false
    }

    // Validate presence option
    #[test]
    fn given_0_int_presence_should_give_never_present() {
        let presence = generate_presence(Some("0"));
        assert!(check_never_present(presence));
    }

    #[test]
    fn given_1_int_presence_should_give_always_present() {
        let presence = generate_presence(Some("1"));
        assert!(check_always_present(presence));
    }

    #[test]
    fn given_between_0_and_1_presence_should_give_sometimes_present() {
        let presence = generate_presence(Some("0.3"));
        assert!(check_sometimes_present(presence));
    }

    #[test]
    fn given_always_should_return_true() {
        let presence = AlwaysPresent;
        assert!(!presence.can_be_null());
        for _ in 1..100 {
            assert!(presence.is_next_present());
        }
    }

    #[test]
    fn given_never_should_return_false() {
        let presence = NeverPresent;
        assert!(presence.can_be_null());
        for _ in 1..100 {
            assert!(!presence.is_next_present());
        }
    }

    #[test]
    fn given_sometimes_should_return_random() {
        let presence = SometimesPresent { presence: 0.5 };
        assert!(presence.can_be_null());

        let mut count = 0;
        for _ in 1..10000 {
            match presence.is_next_present() {
                false => count -= 1,
                true => count += 1,
            };
        }
        assert!(count < 500);
    }
}

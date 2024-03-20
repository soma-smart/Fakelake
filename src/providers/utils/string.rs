use std::iter::repeat_with;

pub fn random_alphanumeric(n: u32) -> String {
    repeat_with(fastrand::alphanumeric)
        .take(n as usize)
        .collect()
}

const EXCLUDE_CHAR_RANGE: u32 = 0xE000 - 0xD800;
const MAX_CHAR: u32 = 0x10FFFF - EXCLUDE_CHAR_RANGE;

pub fn random_characters(n: u32) -> String {
    repeat_with(|| {
        let random_char = fastrand::u32(0..=MAX_CHAR);

        let shifted_char = match random_char {
            result if result >= 0xD800 => result + EXCLUDE_CHAR_RANGE,
            _ => random_char,
        };
        std::char::from_u32(shifted_char).unwrap()
    })
    .take(n as usize)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::{random_alphanumeric, random_characters};
    use regex::Regex;

    fn validate_string(calculated_value: String, length: u32) {
        let pattern = format!(r"^[a-zA-Z0-9]{{{}}}$", length);
        let re = Regex::new(&pattern).unwrap();
        assert!(re.is_match(&calculated_value))
    }

    #[test]
    fn given_0_random_alphanumeric_should_receive_empty_string() {
        assert_eq!(random_alphanumeric(0), "");
    }

    #[test]
    fn given_0_random_characters_should_receive_empty_string() {
        assert_eq!(random_characters(0), "");
    }

    // Validate random calculation
    #[test]
    fn given_x_random_alphanumeric_should_have_string_of_length_x() {
        let values_to_check = [4, 27, 50];
        for length in values_to_check {
            let generated_string = random_alphanumeric(length);
            validate_string(generated_string, length);
        }
    }

    #[test]
    fn given_x_random_characters_should_have_string_of_length_x() {
        let values_to_check = [4, 27, 50];
        for length in values_to_check {
            let generated_string = random_characters(length);
            assert_eq!(generated_string.chars().count(), length.try_into().unwrap());
        }
    }
}

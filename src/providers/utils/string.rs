use std::iter::repeat_with;

pub fn random_characters(n: u32) -> String {
    repeat_with(fastrand::alphanumeric)
        .take(n as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::random_characters;
    use regex::Regex;

    fn validate_string(calculated_value: String, length: u32) {
        let pattern = format!(r"^[a-zA-Z0-9]{{{}}}$", length);
        let re = Regex::new(&pattern).unwrap();
        assert!(re.is_match(&calculated_value))
    }

    #[test]
    fn given_0_should_receive_empty_string() {
        assert_eq!(random_characters(0), "");
    }

    // Validate random calculation
    #[test]
    fn given_x_should_have_string_of_length_x() {
        let values_to_check = [4, 27, 50];
        for length in values_to_check {
            let generated_string = random_characters(length);
            validate_string(generated_string, length);
        }
    }
}

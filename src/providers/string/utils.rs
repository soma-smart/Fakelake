use std::iter::repeat_with;

pub fn random_characters(n: u32) -> String {
    repeat_with(fastrand::alphanumeric).take(n as usize).collect()
}
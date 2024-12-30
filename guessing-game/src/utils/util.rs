use super::log;
use std::cmp::Ordering;

pub fn string_to_i32(input: &str) -> Result<i32, String> {
    input.parse::<i32>().map_err(|error| error.to_string())
}

pub fn check_limits(input: i32) -> bool {
    if !(0..=100).contains(&input) {
        log::invalid_input_for_limits();
        return false;
    }
    true
}

pub fn compare_input_with_target(input: i32, target: i32) -> bool {
    match input.cmp(&target) {
        Ordering::Less => {
            log::input_smaller();
            false
        }
        Ordering::Equal => {
            log::input_equal();
            true
        }
        Ordering::Greater => {
            log::input_greater();
            false
        }
    }
}

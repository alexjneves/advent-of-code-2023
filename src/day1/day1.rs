use crate::day::{Day, InputType, read_day_input};

const DAY_ID: &str = "day1";

pub struct Day1 {}

impl Day for Day1 {
    fn run(&self, input_type: InputType) -> i32 {
        let input = read_day_input(DAY_ID, input_type);

        input.iter()
            .map(|line| get_calibration_value(line))
            .sum()
    }
}

fn get_calibration_value(input: &str) -> i32 {
    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;

    for character in input.chars() {
        if let Some(digit) = character.to_digit(10) {
            let digit_int = Some(i32::try_from(digit).unwrap());

            if first.is_none() {
                first = digit_int;
            }

            last = digit_int;
        }
    }

    first.unwrap() * 10 + last.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_example_input() {
        const EXPECTED_ANSWER: i32 = 142;

        let day1 = Day1 {};
        let answer = day1.run(InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_custom_input() {
        const EXPECTED_ANSWER: i32 = 55607;

        let day1 = Day1 {};
        let answer = day1.run(InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }
}
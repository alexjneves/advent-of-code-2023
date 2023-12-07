use crate::day::{Day, InputType, read_day_input};

const DAY_ID: &str = "day1";

pub struct Day1 {}

impl Day for Day1 {
    fn run(&self, input_type: InputType) -> i32 {
        let input = read_day_input(DAY_ID, input_type);

        println!("Day 1 input: {}", input.as_str());

        return 0;
    }
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
}
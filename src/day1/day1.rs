use crate::day::{Day, Input};

pub struct Day1 {}

impl Day for Day1 {
    fn run(&self, input: Input) -> i32 {
        println!("Day 1 complete");

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_example_inpit() {
        const EXPECTED_ANSWER: i32 = 142;

        let day1 = Day1 {};
        let answer = day1.run(Input::Example);

        assert!(answer == EXPECTED_ANSWER);
    }
}
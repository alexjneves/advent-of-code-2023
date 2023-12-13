use crate::day::{Day, InputType, read_day_input, Part};

const DAY_ID: u8 = 1;

pub struct Day1 {}

impl Day for Day1 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(DAY_ID, &part, &input_type);
        let check_number_text = matches!(part, Part::Two);

        input.iter()
            .map(|line| get_calibration_value(line, check_number_text))
            .sum()
    }
}

fn get_calibration_value(input: &str, check_number_text: bool) -> i32 {
    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;

    for (i, character) in input.chars().enumerate() {
        let mut found_digit: Option<i32> = None;

        if let Some(digit) = character.to_digit(10) {
            found_digit = Some(i32::try_from(digit).unwrap());
        } else if check_number_text { 
            found_digit = is_number_text(&input[i..]);
        }

        if found_digit.is_some() {
            if first.is_none() {
                first = found_digit;
            }

            last = found_digit;
        }
    }

    first.unwrap() * 10 + last.unwrap()
}

fn is_number_text(input: &str) -> Option<i32> {
    if input.len() < 3 {
        return None;
    }

    if !match input.chars().nth(0).unwrap() {
        'o' | 't' | 'f' | 's' | 'e' | 'n' => true,
        _ => false
    } {
        return None;
    }

    if input.len() >= 5 {
        if input.starts_with("three") {
            return Some(3);
        }
    
        if input.starts_with("seven") {
            return Some(7);
        }
    
        if input.starts_with("eight") {
            return Some(8);
        }
    } 
    
    if input.len() >= 4 {
        if input.starts_with("four") {
            return Some(4);
        }
    
        if input.starts_with("five") {
            return Some(5);
        }

        if input.starts_with("nine") {
            return Some(9);
        }
    }

    if input.starts_with("one") {
        return Some(1);
    }

    if input.starts_with("two") {
        return Some(2);
    }

    if input.starts_with("six") {
        return Some(6);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 142;

        let day1 = Day1 {};
        let answer = day1.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 55607;

        let day1 = Day1 {};
        let answer = day1.run(Part::One,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 281;

        let day1 = Day1 {};
        let answer = day1.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 55291;

        let day1 = Day1 {};
        let answer = day1.run(Part::Two,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }
}
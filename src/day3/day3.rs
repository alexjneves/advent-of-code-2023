use std::ops::Range;

use crate::day::{Day, InputType, Part, read_day_input};

const DAY_ID: u8 = 3;

pub struct Day3 {}

impl Day for Day3 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(DAY_ID, &part, &input_type);

        let ep = get_engine_parts(&input);

        ep
            .iter()
            .sum()
    }
}

fn get_engine_parts(lines: &Vec<String>) -> Vec<i32> {
    let char_lines = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut parts: Vec<i32> = vec![];

    for (line_index, char_line) in char_lines.iter().enumerate() {
        let mut processing_number = false;
        let mut part: Vec<i32> = vec![];
        let mut start_index = 0;
        let mut end_index = 0;

        for (char_index, char) in char_line.iter().enumerate() {
            let digit = get_digit(char);

            if let Some(digit) = digit {
                if !processing_number {
                    // found start of a new number
                    start_index = char_index;
                    processing_number = true;
                }

                end_index = char_index;
                part.push(digit);
            } 

            // found a non-digit character or we've reached the end of the line
            // if we were processing a number then evaluate it
            if (digit.is_none() || char_index == char_line.len() - 1) && processing_number {
                let is_engine_part = is_range_adjacent_to_symbol(&char_lines, line_index, start_index..end_index + 1);

                if is_engine_part {
                    let part_value = digits_to_number(&part);
                    parts.push(part_value);
                }

                processing_number = false;
                part.clear();
            }
        }
    }

    parts
}

fn is_range_adjacent_to_symbol(engine: &Vec<Vec<char>>, line_number: usize, range: Range<usize>) -> bool {
    for cell in range {
        let adjacent_cells = get_adjacent_cells(line_number.try_into().unwrap(), cell);

        for (row, col) in adjacent_cells {
            if let Some(cell_is_symbol) = engine.get(row)
                    .and_then(|row_values| row_values.get(col))
                    .and_then(|cell_value| Some(is_symbol(cell_value)))
            {
                if cell_is_symbol {
                    return true;
                }
            }

        }

    }

    false
}

fn get_adjacent_cells(row: usize, col: usize) -> Vec<(usize, usize)> {
    let possible_cells: Vec<(Option<usize>, Option<usize>)> = vec![
        (Some(row), col.checked_sub(1)),
        (row.checked_sub(1), col.checked_sub(1)),
        (row.checked_sub(1), Some(col)),
        (row.checked_sub(1), col.checked_add(1)),
        (Some(row), col.checked_add(1)),
        (row.checked_add(1), col.checked_add(1)),
        (row.checked_add(1), Some(col)),
        (row.checked_add(1), col.checked_sub(1)),
    ];

    possible_cells.iter()
        .filter(|(r, c)| r.is_some() && c.is_some())
        .map(|(r, c)| (r.unwrap(), c.unwrap()))
        .collect()
}

fn get_digit(c: &char) -> Option<i32> {
    c.to_digit(10).and_then(|v| i32::try_from(v).ok())
}

fn digits_to_number(digits: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut multiplier = 1;

    for digit in digits.iter().rev() {
        sum += digit * multiplier;
        multiplier *= 10;
    }

    sum
}

fn is_symbol(char: &char) -> bool {
    *char != '.' && get_digit(char).is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 4361;

        let day3 = Day3 {};
        let answer = day3.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 540025;

        let day3 = Day3 {};
        let answer = day3.run(Part::One, InputType::Custom);

        println!("{}", answer);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day3 = Day3 {};
        let answer = day3.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day3 = Day3 {};
        let answer = day3.run(Part::Two, InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn digit_to_number() {
        assert!(digits_to_number(&vec![1, 2, 3]) == 123);
        assert!(digits_to_number(&vec![9, 8, 7, 6, 3, 2, 1]) == 9876321);
    }
}
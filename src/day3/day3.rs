use std::{ops::Range, collections::HashMap};

use crate::day::{Day, InputType, Part, read_day_input};

const DAY_ID: u8 = 3;

pub struct Day3 {}

impl Day for Day3 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(DAY_ID, &part, &input_type);

        let ep = get_engine_parts(&input);

        if matches!(part, Part::One) {
            return ep
                .iter()
                .map(|part| part.number)
                .sum();
        }

        let potential_gears = ep
            .iter()
            .filter(|part| part.symbol == '*');

        let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

        for potential_gear in potential_gears {
            let gear_values = gears.entry(potential_gear.symbol_coords).or_default();

            gear_values.push(potential_gear.number);
        }

        gears
            .iter()
            .filter(|gear| gear.1.len() == 2)
            .map(|gear| gear.1[0] * gear.1[1])
            .sum()
    }
}

struct EnginePart {
    number: i32,
    symbol: char,
    symbol_coords: (usize, usize)
}

fn get_engine_parts(lines: &Vec<String>) -> Vec<EnginePart> {
    let char_lines = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut parts: Vec<EnginePart> = vec![];

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
                let adjacent_symbol = get_adjacent_symbol(&char_lines, line_index, start_index..end_index + 1);

                if let Some((symbol, coords)) = adjacent_symbol {
                    let part_value = digits_to_number(&part);

                    parts.push(EnginePart { 
                        number: part_value, 
                        symbol, symbol_coords: 
                        coords  });
                }

                processing_number = false;
                part.clear();
            }
        }
    }

    parts
}

fn get_adjacent_symbol(engine: &Vec<Vec<char>>, line_number: usize, range: Range<usize>) -> Option<(char, (usize, usize))> {
    for cell in range {
        let adjacent_cells = get_adjacent_cells(line_number.try_into().unwrap(), cell);

        for (row, col) in adjacent_cells {
            if let Some(symbol) = engine.get(row)
                    .and_then(|row_values| row_values.get(col))
                    .and_then(|cell_value| if is_symbol(cell_value) { Some(cell_value.clone()) } else { None })
            {
                return Some((symbol, (row, col)));
            }
        }
    }

    None
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
        const EXPECTED_ANSWER: i32 = 467835;

        let day3 = Day3 {};
        let answer = day3.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day3_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 84584891;

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
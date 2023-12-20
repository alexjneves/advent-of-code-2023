use regex::Regex;
use lazy_static::lazy_static;

use crate::day::{Day, InputType, Part, read_day_input};

const DAY_ID: u8 = 2;

pub struct Day2 {}

impl Day for Day2 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(DAY_ID, &part, &input_type);

        input.iter()
            .map(parse_game)
            .filter(is_game_valid)
            .map(|game| game.id)
            .sum()
    }
}

lazy_static! {
    static ref GAME_REGEX: Regex = Regex::new(r"Game (\d+): (.+)").unwrap();
    static ref SET_RED_REGEX: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref SET_GREEN_REGEX: Regex = Regex::new(r"(\d+) green").unwrap();
    static ref SET_BLUE_REGEX: Regex = Regex::new(r"(\d+) blue").unwrap();
}

fn parse_game(game: &String) -> Game {
    let captures = GAME_REGEX.captures(game).unwrap();

    let game_id = captures.get(1).unwrap().as_str()
        .parse::<i32>().unwrap();

    let sets: Vec<Set> = captures.get(2).unwrap().as_str()
        .split(';')
        .map(parse_set)
        .collect();

    Game { 
        id: game_id,
        sets
    }
}

fn parse_set(set: &str) -> Set {
    Set {
        red: run_colour_regex(set, &SET_RED_REGEX).unwrap_or(0),
        green: run_colour_regex(set, &SET_GREEN_REGEX).unwrap_or(0),
        blue: run_colour_regex(set, &SET_BLUE_REGEX).unwrap_or(0)
    }
}

fn run_colour_regex(input: &str, regex: &Regex) -> Option<i32> {
    regex.captures(input)?
        .get(1)?
        .as_str()
        .parse::<i32>()
        .ok()
}

fn is_game_valid(game: &Game) -> bool {
    for set in &game.sets {
        if set.red > 12 {
            return false;
        }

        if set.green > 13 {
            return false;
        }

        if set.blue > 14 {
            return false;
        }
    }

    true
}

#[derive(Debug)]
struct Set {
    red: i32,
    green: i32,
    blue: i32
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 8;

        let day2 = Day2 {};
        let answer = day2.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day2 = Day2 {};
        let answer = day2.run(Part::One, InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day2 = Day2 {};
        let answer = day2.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day2 = Day2 {};
        let answer = day2.run(Part::Two, InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }
}
use std::{fs, fmt::Display};

pub trait Day {
    fn run(&self, part: Part, input: InputType) -> i32;
}

pub enum Part {
    One,
    Two
}

pub enum InputType {
    Example,
    Custom
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Example => write!(f, "Example"),
            InputType::Custom => write!(f, "Custom"),
        }
    }
}


pub fn read_day_input(day: u8, part: &Part, input_type: &InputType) -> Vec<String> {
    let path = format!(
        "src/day{}/files/part{}_{}_input.txt", 
        day, 
        part_to_int(part),
        input_type.to_string()
    ).to_lowercase();

    let contents = fs::read_to_string(path).unwrap();

    let lines: Vec<String> = contents
        .split('\n')
        .map(|line| line.to_owned())
        .collect();

    lines
}

fn part_to_int(part: &Part) -> i32 {
    match part {
        Part::One => 1,
        Part::Two => 2,
    }
}
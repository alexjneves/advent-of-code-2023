use std::fs;

pub trait Day {
    fn run(&self, input: InputType) -> i32;
}

pub enum InputType {
    Example,
    Custom
}

pub fn read_day_input(day: &str, input_type: InputType) -> Vec<String> {
    let input_string = match input_type {
        InputType::Example => "example",
        InputType::Custom => "custom"
    };

    let path = format!("src/{}/files/{}_input.txt", day, input_string);

    let contents = fs::read_to_string(path).unwrap();

    let lines: Vec<String> = contents
        .split('\n')
        .map(|line| line.to_owned())
        .collect();

    lines
}
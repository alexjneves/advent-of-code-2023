use std::fs;

pub trait Day {
    fn run(&self, part: Part, input: InputType) -> i32;
}

pub enum Part {
    One,
    Two,
}

pub enum InputType {
    Example,
    Custom,
}

pub fn read_day_input(day: u8, part: &Part, input_type: &InputType) -> Vec<String> {
    let path = match input_type {
        InputType::Example => format!(
            "src/day{}/files/part{}_example_input.txt",
            day,
            part_to_int(part)
        ),
        InputType::Custom => format!(
            "src/day{}/files/custom_input.txt",
            day,
        )
    };

    let contents = fs::read_to_string(path).unwrap();

    contents
        .split('\n')
        .map(|line| line.to_owned())
        .collect()
}

fn part_to_int(part: &Part) -> i32 {
    match part {
        Part::One => 1,
        Part::Two => 2,
    }
}

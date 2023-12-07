mod day;
mod day1;
mod day2;
mod day3;

use clap::Parser;
use day::{Day, InputType};
use day1::day1::Day1;
use day2::day2::Day2;
use day3::day3::Day3;

#[derive(Parser)]
struct Cli {
    day: String,
}

fn main() {
    let args: Cli = Cli::parse();

    let input = InputType::Custom;

    let answer: Result<i32, String> = match args.day.as_str() {
        "day1" => Ok(Day1 {}.run(input)),
        "day2" => Ok(Day2 {}.run(input)),
        "day3" => Ok(Day3 {}.run(input)),
        _ => Err("Invalid day provided".to_owned())
    };

    match answer {
        Ok(answer) => println!("Result: {}", answer),
        Err(error)=> println!("Error: {}", error.as_str()) 
    }
}

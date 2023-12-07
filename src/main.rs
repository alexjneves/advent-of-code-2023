mod day;
mod day1;
mod day2;
mod day3;

use clap::Parser;
use day::Day;
use day1::day1::Day1;
use day2::day2::Day2;
use day3::day3::Day3;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    day: String,
}

fn main() {
    let args: Cli = Cli::parse();

    match args.day.as_str() {
        "day1" => { Day1 {}.run() }
        "day2" => { Day2 {}.run() }
        "day3" => { Day3 {}.run() }
        _ => { println!("unsupported day selected") }
    }
}

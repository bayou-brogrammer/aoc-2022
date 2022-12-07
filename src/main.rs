use clap::Parser;
use common::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "1")]
    day: u8,
}

fn main() {
    let cli = Cli::parse();
    let day = if cli.day > 0 { cli.day } else { 1 };

    let input_file = generate_input_path(day);

    match day {
        1 => day1::day1(input_file),
        2 => day2::day2(input_file),
        3 => day3::day3(input_file),
        4 => day4::day4(input_file),
        5 => day5::day5(input_file),
        6 => day6::day6(input_file),
        day => println!("Day {day} not implemented yet"),
    }
}

fn generate_input_path(day: u8) -> String { format!("./day{day}/{INPUT_FILE}") }

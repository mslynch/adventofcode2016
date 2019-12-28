extern crate adventofcode2016;
#[macro_use]
extern crate clap;

use adventofcode2016::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, solution::Solution,
};
use clap::{App, Arg};
use std::fs::File;

fn main() {
    let matches = App::new("Advent of Code 2016")
        .version(crate_version!())
        .about("Runs my solutions to the Advent of Code 2016 problems.")
        .author("Matthew Lynch")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("The day to run solutions for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILENAME")
                .help("The input file to run the solution on")
                .takes_value(true),
        )
        .get_matches();

    let day_input = matches.value_of("day").unwrap();

    let day = match day_input.chars().count() {
        1 => format!("0{}", day_input),
        _ => day_input.to_string(),
    };

    let default_filename = &format!("data/day{}/input.dat", day);
    let filename = matches.value_of("file").unwrap_or(default_filename);

    let mut file = File::open(filename).expect("File not found!");

    let runner_result: Result<fn(&mut File) -> Solution, &str> = match day.as_ref() {
        "01" => Ok(day01::run),
        "02" => Ok(day02::run),
        "03" => Ok(day03::run),
        "04" => Ok(day04::run),
        "05" => Ok(day05::run),
        "06" => Ok(day06::run),
        "07" => Ok(day07::run),
        "08" => Ok(day08::run),
        "09" => Ok(day09::run),
        "10" => Ok(day10::run),
        "11" => Ok(day11::run),
        "12" => Ok(day12::run),
        "13" => Ok(day13::run),
        "14" => Ok(day14::run),
        "15" => Ok(day15::run),
        "16" => Ok(day16::run),
        "17" => Ok(day17::run),
        "18" => Ok(day18::run),
        // "19" => Ok(day19::run),
        // "20" => Ok(day20::run),
        // "21" => Ok(day21::run),
        // "22" => Ok(day22::run),
        // "23" => Ok(day23::run),
        // "24" => Ok(day24::run),
        // "25" => Ok(day25::run),
        _ => Err("Enter a day between 1 and 25!"),
    };
    let runner = runner_result.unwrap();

    let solution = runner(&mut file);
    println!("Day {}: {}", day_input, solution.title);
    println!("part 1:\n{}", solution.part1);
    println!("part 2:\n{}", solution.part2);
}

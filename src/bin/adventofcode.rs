extern crate adventofcode2016;
#[macro_use]
extern crate clap;

use adventofcode2016::{day01, day02, day03, day04, day05, day06, day07, day08, day09};
use clap::{App, Arg};

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
        ).arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILENAME")
                .help("The input file to run the solution on")
                .takes_value(true),
        ).get_matches();
    match matches.value_of("day").unwrap() {
        "1" => day01::run(matches.value_of("file")),
        "2" => day02::run(matches.value_of("file")),
        "3" => day03::run(matches.value_of("file")),
        "4" => day04::run(matches.value_of("file")),
        "5" => day05::run(matches.value_of("file")),
        "6" => day06::run(matches.value_of("file")),
        "7" => day07::run(matches.value_of("file")),
        "8" => day08::run(matches.value_of("file")),
        "9" => day09::run(matches.value_of("file")),
        // "10" => day10::run(matches.value_of("file")),
        // "11" => day11::run(matches.value_of("file")),
        // "12" => day12::run(matches.value_of("file")),
        // "13" => day13::run(matches.value_of("file")),
        // "14" => day14::run(matches.value_of("file")),
        // "15" => day15::run(matches.value_of("file")),
        // "16" => day16::run(matches.value_of("file")),
        // "17" => day17::run(matches.value_of("file")),
        // "18" => day18::run(matches.value_of("file")),
        // "19" => day19::run(matches.value_of("file")),
        // "20" => day20::run(matches.value_of("file")),
        // "21" => day21::run(matches.value_of("file")),
        // "22" => day22::run(matches.value_of("file")),
        // "23" => day23::run(matches.value_of("file")),
        // "24" => day24::run(matches.value_of("file")),
        // "25" => day25::run(matches.value_of("file")),
        _ => println!("Enter a day between 1 and 25!"),
    }
}

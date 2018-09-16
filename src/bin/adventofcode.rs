extern crate adventofcode2016;
#[macro_use]
extern crate clap;

use adventofcode2016::{day01, day02, day03, day04};
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
        _ => println!("Enter a day between 1 and 25!"),
    }
}

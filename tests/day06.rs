extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day06::run;

#[test]
fn error_correction_max_test() {
    let mut file = File::open("data/day06/test.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("easter", result, "error-corrected message is found");
}

#[test]
fn error_correction_min_test() {
    let mut file = File::open("data/day06/test.dat").expect("File not found!");
    let result = run(&mut file).part2;
    assert_eq!(
        "advent", result,
        "error-corrected message is found when least common letter is chosen"
    );
}

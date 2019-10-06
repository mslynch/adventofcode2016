extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day04::run;

#[test]
fn real_rooms_test() {
    let mut file = File::open("data/day04/test-real-rooms.dat").expect("File not found!");
    let result = run(&mut file).part1;
    let expected = (123 + 987 + 404).to_string();
    assert_eq!(expected, result, "real sector ids are summed");
}

#[test]
fn northpole_test() {
    let mut file = File::open("data/day04/test-northpole.dat").expect("File not found!");
    let result = run(&mut file).part2;
    assert_eq!("53", result, "north pole's sector id is found");
}

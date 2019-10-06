extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day01::run;

#[test]
fn test1() {
    let mut file = File::open("data/day01/test1.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("5", result);
}

#[test]
fn test2() {
    let mut file = File::open("data/day01/test2.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("2", result);
}

#[test]
fn test3() {
    let mut file = File::open("data/day01/test3.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("12", result);
}

#[test]
fn test4() {
    let mut file = File::open("data/day01/test4.dat").expect("File not found!");
    let result = run(&mut file).part2;
    assert_eq!("4", result);
}

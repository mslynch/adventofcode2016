extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day03::run;

#[test]
fn actual_triangle_count_test() {
    let mut file = File::open("data/day03/test.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("3", result, "number of triangles is calculated");
}

#[test]
fn actual_vertical_triangle_count_test() {
    let mut file = File::open("data/day03/test.dat").expect("File not found!");
    let result = run(&mut file).part2;
    assert_eq!(
        "3", result,
        "number of triangles is calculated when input is parsed vertically"
    );
}

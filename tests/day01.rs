extern crate adventofcode2016;

use adventofcode2016::day01::blocks_away;

#[test]
fn blocks_away_test_1() {
    let (result, _) = blocks_away("R2, L3");
    assert_eq!(result, 5);
}

#[test]
fn blocks_away_test_2() {
    let (result, _) = blocks_away("R2, R2, R2");
    assert_eq!(result, 2);
}

#[test]
fn blocks_away_test_3() {
    let (result, _) = blocks_away("R5, L5, R5, R3");
    assert_eq!(result, 12);
}

#[test]
fn test_revisit_blocks_away() {
    let (_, result) = blocks_away("R8, R4, R4, R8");
    assert_eq!(result.unwrap(), 4);
}

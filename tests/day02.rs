extern crate adventofcode2016;

use adventofcode2016::day02::{decode_instructions, Coord, KEYPAD_DIAMOND, KEYPAD_NORMAL};

#[test]
fn normal_decode_test() {
    let input = [
        "ULL".to_string(),
        "RRDDD".to_string(),
        "LURDL".to_string(),
        "UUUUD".to_string(),
    ];

    assert_eq!(
        "1985",
        decode_instructions(&input, KEYPAD_NORMAL, Coord { row: 1, col: 1 }, Coord::next)
    );
}

#[test]
fn diamond_decode_test() {
    let input = [
        "ULL".to_string(),
        "RRDDD".to_string(),
        "LURDL".to_string(),
        "UUUUD".to_string(),
    ];

    assert_eq!(
        "5DB3",
        decode_instructions(
            &input,
            KEYPAD_DIAMOND,
            Coord { row: 2, col: 0 },
            Coord::next_diamond
        )
    );
}

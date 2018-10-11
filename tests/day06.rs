extern crate adventofcode2016;

use adventofcode2016::day06::{error_correct, max_count_comparator, min_count_comparator};

fn input() -> Vec<String> {
    vec![
        "eedadn".to_string(),
        "drvtee".to_string(),
        "eandsr".to_string(),
        "raavrd".to_string(),
        "atevrs".to_string(),
        "tsrnev".to_string(),
        "sdttsa".to_string(),
        "rasrtv".to_string(),
        "nssdts".to_string(),
        "ntnada".to_string(),
        "svetve".to_string(),
        "tesnvt".to_string(),
        "vntsnd".to_string(),
        "vrdear".to_string(),
        "dvrsen".to_string(),
        "enarar".to_string(),
    ]
}

#[test]
fn error_correction_max_test() {
    assert_eq!("easter", error_correct(&input(), max_count_comparator));
}

#[test]
fn error_correction_min_test() {
    assert_eq!("advent", error_correct(&input(), min_count_comparator));
}

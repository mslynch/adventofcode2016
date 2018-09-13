extern crate adventofcode2016;

use adventofcode2016::day03::actual_triangle_count;

#[test]
fn actual_triangle_count_test() {
    let input = vec![
        "3 4 5".to_string(),
        "3 3 3".to_string(),
        "3 4 4".to_string(),
        "5 10 25".to_string(),
    ];

    assert_eq!(
        3,
        actual_triangle_count(input)
    );
}

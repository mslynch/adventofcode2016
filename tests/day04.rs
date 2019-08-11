extern crate adventofcode2016;

use adventofcode2016::day04::real_room_id_sum;

#[test]
fn actual_triangle_count_test() {
    let input = vec![
        "aaaaa-bbb-z-y-x-123[abxyz]".to_string(),
        "a-b-c-d-e-f-g-h-987[abcde]".to_string(),
        "not-a-real-room-404[oarel]".to_string(),
        "totally-real-room-200[decoy]".to_string(),
    ];

    assert_eq!(123 + 987 + 404, real_room_id_sum(&input));
}

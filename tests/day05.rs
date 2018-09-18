extern crate adventofcode2016;

use adventofcode2016::day05::generate_password;

#[test]
fn password_generation_test() {
    assert_eq!("18f47a30", generate_password("abc"));
}


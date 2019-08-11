extern crate adventofcode2016;

use adventofcode2016::day05::{generate_password, generate_password_2};

#[test]
fn password_generation_test() {
    assert_eq!("18f47a30", generate_password("abc"));
}

#[test]
fn password_generation_2_test() {
    assert_eq!("05ace8e3", generate_password_2("abc"));
}

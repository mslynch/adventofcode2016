extern crate adventofcode2016;

use std::fs::File;

use adventofcode2016::day07::run;

#[test]
fn tls_test() {
    let mut file = File::open("data/day07/test-tls.dat").expect("File not found!");
    let result = run(&mut file).part1;
    assert_eq!("2", result, "TLS-supporting IPs are identified");
}

#[test]
fn ssl_test() {
    let mut file = File::open("data/day07/test-ssl.dat").expect("File not found!");
    let result = run(&mut file).part2;
    assert_eq!("3", result, "SSL-supporting IPs are identified");
}

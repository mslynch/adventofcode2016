extern crate adventofcode2016;

use adventofcode2016::day07::{parse_address, spy_support_count, IPAddress};

fn input_1() -> Vec<String> {
    vec![
        "abba[mnop]qrst".to_string(),
        "abcd[bddb]xyyx".to_string(),
        "aaaa[qwer]tyui".to_string(),
        "ioxxoj[asdfgh]zxcvbn".to_string(),
    ]
}

fn input_2() -> Vec<String> {
    vec![
        "aba[bab]xyz".to_string(),
        "xyx[xyx]xyx".to_string(),
        "aaa[kek]eke".to_string(),
        "zazbz[bzb]cdb".to_string(),
    ]
}

#[test]
fn spy_support_count_test() {
    assert_eq!(2, spy_support_count(&input_1(), IPAddress::supports_tls));
}

#[test]
fn tls_test_1() {
    assert!(parse_address(&input_1()[0]).supports_tls());
}

#[test]
fn tls_test_2() {
    assert!(!parse_address(&input_1()[1]).supports_tls());
}

#[test]
fn tls_test_3() {
    assert!(!parse_address(&input_1()[2]).supports_tls());
}

#[test]
fn tls_test_4() {
    assert!(parse_address(&input_1()[3]).supports_tls());
}

#[test]
fn ssl_test_1() {
    assert!(parse_address(&input_2()[0]).supports_ssl());
}

#[test]
fn ssl_test_2() {
    assert!(!parse_address(&input_2()[1]).supports_ssl());
}

#[test]
fn ssl_test_3() {
    assert!(parse_address(&input_2()[2]).supports_ssl());
}

#[test]
fn ssl_test_4() {
    assert!(parse_address(&input_2()[3]).supports_ssl());
}

use solution::Solution;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

/// Runs the solutions for day 7.
pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);

    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();

    Solution {
        title: "Internet Protocol Version 7".to_string(),
        part1: spy_support_count(&input, IPAddress::supports_tls).to_string(),
        part2: spy_support_count(&input, IPAddress::supports_ssl).to_string(),
    }
}

/// Returns the number of IP addresses that support the given spy protocol.
fn spy_support_count<F>(input: &[String], spy_support_predicate: F) -> usize
where
    F: Fn(&IPAddress) -> bool,
{
    input
        .iter()
        .map(|s| parse_address(s))
        .filter(|addr| spy_support_predicate(addr))
        .count()
}

#[derive(PartialEq, Debug)]
struct IPAddress {
    sequences: Vec<String>,
    hypernet_sequences: Vec<String>,
}

impl IPAddress {
    fn supports_tls(&self) -> bool {
        self.sequences.iter().any(|s| contains_abba(s))
            && self.hypernet_sequences.iter().all(|s| !contains_abba(s))
    }

    fn supports_ssl(&self) -> bool {
        self.sequences
            .iter()
            .flat_map(|s| bab_strings(s))
            .map(|s| invert_bab(s))
            .any(|s| self.hypernet_sequences.iter().any(|hs| hs.contains(&s)))
    }
}

/// Transforms a &str 'bab' into 'aba'.
fn invert_bab(bab: &str) -> String {
    let mut ret = String::new();
    let mut bab_chars = bab.chars();
    let first = bab_chars.next().unwrap();
    let second = bab_chars.next().unwrap();
    ret.push(second);
    ret.push(first);
    ret.push(second);
    ret
}

/// Turns a &str into an IPAddress.
fn parse_address(string: &str) -> IPAddress {
    let mut sequences = vec!["".to_string()];
    let mut hypernet_sequences = Vec::new();
    let mut in_hypernet = false;
    for c in string.chars() {
        match c {
            '[' => {
                in_hypernet = true;
                hypernet_sequences.push("".to_string());
            }
            ']' => {
                in_hypernet = false;
                sequences.push("".to_string());
            }
            _ => {
                let working_sequence = if in_hypernet {
                    &mut hypernet_sequences
                } else {
                    &mut sequences
                };
                working_sequence.last_mut().unwrap().push(c);
            }
        }
    }

    IPAddress {
        sequences,
        hypernet_sequences,
    }
}

/// Checks whether a four-character String is a palindrome and not all the characters are the same.
fn contains_abba(string: &str) -> bool {
    string
        .as_bytes()
        .windows(4)
        .any(|s| s[0] == s[3] && s[1] == s[2] && s[0] != s[1])
}

/// Finds all 'bab' substrings in a String.
fn bab_strings(string: &str) -> impl Iterator<Item = &str> {
    string
        .as_bytes()
        .windows(3)
        .filter(|s| s[0] == s[2] && s[0] != s[1])
        .map(|s| str::from_utf8(&s).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_address_test_1() {
        assert_eq!(
            IPAddress {
                sequences: vec!["aaaa".to_string(), "cccc".to_string(), "eeee".to_string()],
                hypernet_sequences: vec!["bbbb".to_string(), "dddd".to_string()],
            },
            parse_address("aaaa[bbbb]cccc[dddd]eeee")
        );
    }

    #[test]
    fn contains_abba_test_1() {
        assert!(contains_abba("abba"))
    }

    #[test]
    fn contains_abba_test_2() {
        assert!(contains_abba("aabbab"))
    }

    #[test]
    fn non_abba_test_1() {
        assert!(!contains_abba("aaaa"))
    }

    #[test]
    fn non_abba_test_2() {
        assert!(!contains_abba("abbb"))
    }

    #[test]
    fn non_abba_test_3() {
        assert!(!contains_abba("abcdef"))
    }

    #[test]
    fn bab_strings_test() {
        assert_eq!(
            vec!["bab", "fgf"],
            bab_strings("babfgf").collect::<Vec<&str>>()
        )
    }

    #[test]
    fn tls_test_1() {
        assert!(parse_address("abba[mnop]qrst").supports_tls());
    }

    #[test]
    fn tls_test_2() {
        assert!(!parse_address("abcd[bddb]xyyx").supports_tls());
    }

    #[test]
    fn tls_test_3() {
        assert!(!parse_address("aaaa[qwer]tyui").supports_tls());
    }

    #[test]
    fn tls_test_4() {
        assert!(parse_address("ioxxoj[asdfgh]zxcvbn").supports_tls());
    }

    #[test]
    fn ssl_test_1() {
        assert!(parse_address("aba[bab]xyz").supports_ssl());
    }

    #[test]
    fn ssl_test_2() {
        assert!(!parse_address("xyx[xyx]xyx").supports_ssl());
    }

    #[test]
    fn ssl_test_3() {
        assert!(parse_address("aaa[kek]eke").supports_ssl());
    }

    #[test]
    fn ssl_test_4() {
        assert!(parse_address("zazbz[bzb]cdb").supports_ssl());
    }
}

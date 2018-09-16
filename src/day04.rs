use itertools::Itertools;

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::sorted;

pub fn run(filename: Option<&str>) {
    println!("Day 4: Security Through Obscurity");
    let file = File::open(filename.unwrap_or("data/day04.txt")).expect("file not found");
    let reader = BufReader::new(file);

    let input = reader.lines().map(Result::unwrap).collect::<Vec<String>>();

    println!("part 1: {}", real_room_id_sum(&input));
    // find_north_pole_sector(&input);
    println!("part 2: {}", find_north_pole_sector(&input));
}

#[derive(PartialEq, Debug)]
struct Room<'a> {
    words: Vec<&'a str>,
    id: usize,
    checksum: String,
}

#[derive(Eq, PartialEq, PartialOrd)]
struct CharCount {
    character: char,
    count: usize,
}

impl Ord for CharCount {
    fn cmp(&self, other: &CharCount) -> Ordering {
        match self.count.cmp(&other.count) {
            Ordering::Equal => self.character.cmp(&other.character),
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
        }
    }
}

pub fn real_room_id_sum(input: &[String]) -> usize {
    input
        .iter()
        .map(|input| parse_room(input))
        .filter(Room::is_real)
        .map(|room| room.id)
        .sum()
}

pub fn find_north_pole_sector(input: &[String]) -> usize {
    let (id, _decrypted) = input
        .iter()
        .map(|input| parse_room(input))
        .filter(Room::is_real)
        .map(|room| (room.id, room.decrypt()))
        .find(|(_id, decrypted)| decrypted.contains("northpole"))
        .unwrap();
    id
}

impl<'a> Room<'a> {
    fn is_real(&self) -> bool {
        let mut chars_by_count = BTreeMap::new();
        for c in self.words.iter().flat_map(|s| s.chars()) {
            let count = chars_by_count.entry(c).or_insert(0);
            *count += 1;
        }
        let sorted = sorted(chars_by_count.iter().map(|(c, count)| CharCount {
            character: *c,
            count: *count,
        }));
        sorted
            .iter()
            .map(|char_count| char_count.character)
            .take(5)
            .collect::<String>()
            == self.checksum
    }

    fn decrypt(&self) -> String {
        let shift = (self.id % 26) as u8;
        self.words
            .iter()
            .map(|word| {
                word.chars()
                    .map(|c| shift_char(c, shift))
                    .collect::<String>()
            }).join(" ")
    }
}

fn parse_room(room: &str) -> Room {
    let size = room.len();
    let checksum = room[size - 6..size - 1].to_string();
    let mut id_name_iter = room[..size - 7].split('-').rev();
    let id: usize = id_name_iter.next().unwrap().parse().unwrap();
    let words: Vec<&str> = id_name_iter.rev().collect();
    Room {
        words,
        id,
        checksum,
    }
}

fn shift_char(c: char, shift: u8) -> char {
    let shifted = c as u8 + shift;
    if shifted <= 122 {
        shifted as char
    } else {
        (shifted - 26) as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_parsing() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]";
        let expected_room = Room {
            words: vec!["aaaaa", "bbb", "z", "y", "x"],
            id: 123,
            checksum: "abxyz".to_string(),
        };
        assert_eq!(expected_room, parse_room(input));
    }

    #[test]
    fn test_shift_char_1() {
        assert_eq!('a', shift_char('a', 0));
    }

    #[test]
    fn test_shift_char_2() {
        assert_eq!('e', shift_char('a', 4));
    }

    #[test]
    fn test_shift_char_3() {
        assert_eq!('a', shift_char('z', 1));
    }

    #[test]
    fn test_shift_char_4() {
        assert_eq!('z', shift_char('y', 1));
    }

    #[test]
    fn decryption_test() {
        let room = Room {
            words: vec!["qzmt", "zixmtkozy", "ivhz"],
            id: 343,
            checksum: "".to_string(),
        };
        assert_eq!("very encrypted name", room.decrypt());
    }

}

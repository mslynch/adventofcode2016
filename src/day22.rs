use solution::Solution;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
Part 2: Approach as a 15 puzzle. Slide empty space _ to move G to 0.

0.......c...........................G
.....................................
.....................................
.....................................
.........############################
........b.............a..............
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
.....................................
......................_..............
.....................................
.....................................

to a:  20
to b:  14
to c:  5
to G:  28
to 0:  35 * 5
total: 242
*/

pub fn run(file: &mut File) -> Solution {
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let nodes_to_usage = parse_input(&input);

    let viable_pairs = get_viable_pairs(&nodes_to_usage);

    Solution {
        title: "Scrambled Letters and Hash".to_string(),
        part1: viable_pairs.len().to_string(),
        part2: string_representation(&nodes_to_usage),
    }
}

fn parse_input(input: &[String]) -> HashMap<Coord, DiskUsage> {
    let mut iter = input.iter();
    iter.next();
    iter.next();
    iter.map(|line| {
        let mut split = line.split_whitespace();
        let node = split.next().unwrap();
        let size = split.next().unwrap();
        let size_i32 = size[0..size.len() - 1].to_string().parse::<i32>().unwrap();
        let used = split.next().unwrap();
        let used_i32 = used[0..used.len() - 1].to_string().parse::<i32>().unwrap();
        let mut node_data = node.split('-');
        node_data.next();
        let x = node_data.next().unwrap()[1..]
            .to_string()
            .parse::<i32>()
            .unwrap();
        let y = node_data.next().unwrap()[1..]
            .to_string()
            .parse::<i32>()
            .unwrap();
        let coord = Coord { x, y };
        let disk_usage = DiskUsage {
            size: size_i32,
            used: used_i32,
        };
        (coord, disk_usage)
    })
    .collect()
}

fn get_viable_pairs(nodes_to_usage: &HashMap<Coord, DiskUsage>) -> Vec<CoordPair> {
    nodes_to_usage
        .iter()
        .flat_map(|(coord, _usage)| {
            nodes_to_usage.keys().map(move |b_coord| CoordPair {
                a: *coord,
                b: b_coord.to_owned(),
            })
        })
        .filter(|coord_pair| {
            let used_by_a = nodes_to_usage.get(&coord_pair.a).unwrap().used;
            let size_of_b = nodes_to_usage.get(&coord_pair.b).unwrap().size;
            let used_by_b = nodes_to_usage.get(&coord_pair.b).unwrap().used;
            let available_on_b = size_of_b - used_by_b;
            used_by_a != 0 && coord_pair.a != coord_pair.b && used_by_a <= available_on_b
        })
        .collect()
}

fn string_representation(nodes_to_usage: &HashMap<Coord, DiskUsage>) -> String {
    let max_x = nodes_to_usage.keys().max_by_key(|coord| coord.x).unwrap().x;
    let max_y = nodes_to_usage.keys().max_by_key(|coord| coord.y).unwrap().y;
    (0..=max_y)
        .flat_map(move |y| {
            (0..=max_x)
                .map(move |x| {
                    if x == max_x && y == 0 {
                        return 'G';
                    }
                    let disk_usage = nodes_to_usage.get(&Coord { x, y }).unwrap();
                    if disk_usage.used == 0 {
                        '_'
                    } else if disk_usage.size > 500 {
                        '#'
                    } else {
                        '.'
                    }
                })
                .chain(std::iter::once('\n'))
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

struct CoordPair {
    a: Coord,
    b: Coord,
}

#[derive(Debug)]
struct DiskUsage {
    size: i32,
    used: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_position_test() {
        assert_eq!("acbd".to_string(), swap_position("abcd", 1, 2));
    }
}

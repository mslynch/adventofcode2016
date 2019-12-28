use solution::Solution;
use std::fs::File;
use std::io::prelude::*;

/// Runs the solutions for day 17.
pub fn run(file: &mut File) -> Solution {
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    Solution {
        title: "Two Steps Forward".to_string(),
        part1: find_shortest_path(&input).to_string(),
        part2: find_longest_path_length(&input).to_string(),
    }
}

#[derive(PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn find_shortest_path(input: &str) -> String {
    let mut working_paths: Vec<String> = vec!["".to_string()];
    let mut path = None;
    while path == None {
        working_paths = get_new_paths(&working_paths, input);
        path = working_paths.iter().find(|path| is_finished(path));
    }
    path.unwrap().to_string()
}

fn find_longest_path_length(input: &str) -> usize {
    let mut working_paths: Vec<String> = vec!["".to_string()];
    let mut longest_length = 0;
    while !working_paths.is_empty() {
        working_paths = get_new_paths(&working_paths, input);
        working_paths.retain(|path| {
            let should_retain = !is_finished(path);
            if !should_retain {
                longest_length = path.len();
            }
            should_retain
        });
    }
    longest_length
}

fn get_new_paths(paths: &[String], input: &str) -> Vec<String> {
    paths
        .iter()
        .flat_map(|path| {
            get_open_directions(&path, input)
                .into_iter()
                .map(move |direction| format!("{}{}", &path, &direction))
        })
        .collect()
}

fn is_finished(path: &str) -> bool {
    let cond = get_position(path) == Position { x: 3, y: 3 };
    if cond {}
    cond
}

fn is_door_open(c: char) -> bool {
    ('b'..='f').contains(&c)
}

fn get_open_directions(path: &str, input: &str) -> Vec<char> {
    let position = get_position(&path);
    let hash_input = format!("{}{}", input, path);
    let hash = format!("{:x}", md5::compute(hash_input));
    let mut hash_chars = hash.chars();
    let mut open_paths = vec![];
    if is_door_open(hash_chars.next().unwrap()) && position.y > 0 {
        open_paths.push('U');
    }
    if is_door_open(hash_chars.next().unwrap()) && position.y < 3 {
        open_paths.push('D');
    }
    if is_door_open(hash_chars.next().unwrap()) && position.x > 0 {
        open_paths.push('L');
    }
    if is_door_open(hash_chars.next().unwrap()) && position.x < 3 {
        open_paths.push('R');
    }
    open_paths
}

fn get_position(path: &str) -> Position {
    let mut x = 0;
    let mut y = 0;
    for c in path.chars() {
        if c == 'U' {
            y -= 1;
        } else if c == 'D' {
            y += 1;
        } else if c == 'L' {
            x -= 1;
        } else {
            x += 1;
        }
    }
    Position { x, y }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_position_test() {
        let final_position = Position { x: 3, y: 3 };
        assert_eq!(get_position("DDRRRD"), final_position);
        assert_eq!(get_position("DDUDRLRRUDRD"), final_position);
        assert_eq!(
            get_position("DRURDRUDDLLDLUURRDULRLDUUDDDRR"),
            final_position
        );
        let path = "DRURDRUDDLLDLUURRDULRLDUDRRLUDDLUULDRDURDURLLDRUDUDURLUDLLURULRRDDDLULUUDURRDLLUDDRULRDUUDDLUUDRDRLURLULDDRUDURDDULLURURRLDLDURUDDURDUDLLURDUDURLDRULRLULRLDRLRRLLLRDDRURLUULDUDDUULDRDULRUDRULRDLRUDLUDLRRLDLUDRRRULDULRDLLDRRUUDLRLRDULUDUDURLLRRDRLUULLRRRLLDRDURLDLRURLLDDLURURULRRLLDDUDUURLDRDLRLDRULURULDRDULRURLLLRLDDRURDDLUDUURLRULRDULLRLDDURDLRLRURRDLULRDUDRLDLUUDLURRURLDRDLULRDURLRLLUDULRRDUDURLDULRLDLRLRULRDRULDLRURDDRUUDDLUULDLUDURDRDLLRLDUUDRDLRUDURLRLDRLLUDRURURDLLURRLDLLURLDUDUDRRURLDUUDDULDUDDUUDUUDDRUULDLRDRLDRULDRURLLUDUUDLRURLDRLUDUDRRDLLURLDUURDLLRDLRRLURRDULDRULLRUDDLDLUDRURURLDUDDULLDUURRDDULRRULDLDURLRLRULRDUURLRDDUULLDURDLDRDURLUDDUDUULRLRDLUUDURLDDDULDUUDURLRUDRRLULLRDLDRURLRRDLRLLLRUDRDLLRURULRDDULUDLURLRLRDDLUURLRUDLRUDURDLDUUDLDRUUDLURDDDUURULDRULRLLDDRRDLURUDDUUDLDLRUDRULRLLRULUDUDDDURLRRUDUDDLURDR";
        assert_eq!(get_position(path), final_position);
    }
}

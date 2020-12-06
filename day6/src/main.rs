use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn parse_input() -> String {
    let mut file = File::open("input.txt").expect("input.txt does not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read contents to string");
    contents
}

fn parse_sets(input: &str) -> Vec<HashSet<char>> {
    let mut sets: Vec<HashSet<char>> = Vec::new();
    let mut set: HashSet<char> = HashSet::new();
    for line in input.lines() {
        if line.is_empty() {
            sets.push(set);
            set = HashSet::new();
        } else {
            for item in line.chars() {
                set.insert(item);
            }
        }
    }
    sets.push(set);
    sets
}

fn sum_counts(sets: Vec<HashSet<char>>) -> usize {
    sets.iter().map(|s| s.len()).sum()
}

fn main() {
    let input = parse_input();
    println!("sum: {}", sum_counts(parse_sets(&input)));
}

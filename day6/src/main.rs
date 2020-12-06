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

fn parse_sets_intersection(input: &str) -> Vec<HashSet<char>> {
    let mut sets: Vec<HashSet<char>> = Vec::new();
    let mut working_sets: Vec<HashSet<char>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            let mut current_set = HashSet::new();
            current_set = current_set.union(&working_sets[0]).map(|e| *e).collect();
            for i in 1..working_sets.len() {
                current_set = current_set.intersection(&working_sets[i]).map(|e| *e).collect();
            }
            sets.push(current_set);
            working_sets = Vec::new();
        } else {
            let mut new_set: HashSet<char> = HashSet::new();
            for item in line.chars() {
                new_set.insert(item);
            }
            working_sets.push(new_set);
        }
    }
    let mut current_set = HashSet::new();
    current_set = current_set.union(&working_sets[0]).map(|e| *e).collect();
    for i in 1..working_sets.len() {
        current_set = current_set.intersection(&working_sets[i]).map(|e| *e).collect();
    }
    sets.push(current_set);
    sets
}

fn sum_counts(sets: Vec<HashSet<char>>) -> usize {
    sets.iter().map(|s| s.len()).sum()
}

fn main() {
    let input = parse_input();
    println!("sum: {}", sum_counts(parse_sets(&input)));
    println!("intersection sum: {}", sum_counts(parse_sets_intersection(&input)));
}

use std::fs::File;
use std::io::prelude::*;

fn parse_input() -> String {
    let mut file = File::open("input.txt").expect("input.txt does not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read contents to string");
    contents
}

fn calculate_seat_id(pass: &str) -> usize {
    let mut min_row = 0;
    let mut max_row = 127;
    let items: Vec<char> = pass.chars().collect();
    for i in 0..7 {
        if items[i] == 'F' {
            max_row = max_row - ((max_row - min_row) / 2 + 1);
        } else {
            min_row = min_row + ((max_row - min_row) / 2 + 1);
        }
    }
    let mut min_col = 0;
    let mut max_col = 7;
    for i in 7..10 {
        println!("{}", items[i]);
        if items[i] == 'L' {
            max_col = max_col - (((max_col - min_col) / 2) + 1);
        } else {
            min_col = min_col + ((max_col - min_col) / 2 + 1);
        }
    }
    max_row * 8 + max_col
}

fn find_highest_seat_id(passes: &str) -> usize {
    let mut highest = 0;
    for pass in passes.lines() {
        let seat_id = calculate_seat_id(pass);
        if seat_id > highest {
            highest = seat_id;
        }
    }
    highest
}

fn calculate_all_seat_ids(passes: &str) -> Vec<usize> {
    let mut ids: Vec<usize> = Vec::new();
    for pass in passes.lines() {
        let seat_id = calculate_seat_id(pass);
        ids.push(seat_id);
    }
    ids.sort();
    ids
}

fn find_missing_seat_id(ids: Vec<usize>) -> usize {
    let mut missing_seat = 0;
    for i in 0..ids.len() - 1 {
        if ids[i + 1] != ids[i] + 1 {
            missing_seat = ids[i] + 1;
            break;
        }
    }
    missing_seat
}

fn main() {
    let input = parse_input();
    println!("highest seat: {}", find_highest_seat_id(&input));
    let ids = calculate_all_seat_ids(&input);
    println!("my seat: {}", find_missing_seat_id(ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity() {
        assert_eq!(calculate_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(calculate_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(calculate_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(calculate_seat_id("BBFFBBFRLL"), 820);
        assert_eq!(
            find_highest_seat_id("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL"),
            820
        );
    }
}

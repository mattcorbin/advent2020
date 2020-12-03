use std::fs::File;
use std::io::prelude::*;

fn parse_input() -> String {
    let mut file = File::open("input.txt").expect("input.txt does not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read contents to string");
    contents
}

fn slide(start: usize, distance: usize, max: usize) -> usize {
    (start + distance) % max
}

fn filter_lines(dist: usize, idx: usize, e: &str) -> Option<&str> {
    if idx == 0 {
        None
    } else if idx%dist != 0 {
        None
    } else {
        Some(e)
    }
}

fn count_trees(map: &str, h_move: usize, v_move: usize) -> usize {
    let mut hits = 0;
    let mut horizontal = h_move;
    let lines: Vec<&str> = map.lines().enumerate().filter_map(|(i, e)| filter_lines(v_move, i, e)).collect();
    for line in lines {
        let char_vec: Vec<char> = line.chars().collect();
        if char_vec[horizontal] == '#' {
            hits += 1;
        }
        horizontal = slide(horizontal, h_move, char_vec.len());
    }
    hits
}

fn main() {
    let input = parse_input();
    println!("trees hit: {}", count_trees(&input, 3, 1));

    let tests = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut results: Vec<usize> = Vec::new();
    for test in tests.iter() {
        results.push(count_trees(&input, test.0, test.1));
    }
    println!("product: {}", results.iter().product::<usize>());
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    #[test]
    fn test_count_trees() {
        assert_eq!(count_trees(INPUT, 1, 1), 2);
        assert_eq!(count_trees(INPUT, 3, 1), 7);
        assert_eq!(count_trees(INPUT, 5, 1), 3);
        assert_eq!(count_trees(INPUT, 7, 1), 4);
        assert_eq!(count_trees(INPUT, 1, 2), 2);
    }

}
use std::fs::File;
use std::io::prelude::*;

fn parse_input() -> Vec<i64> {
    let mut retval: Vec<i64> = Vec::new();
    let mut file = File::open("input.txt").expect("input.txt does not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read contents to string");
    for item in contents.lines() {
        retval.push(item.parse().expect("failed to parse string to i64"));
    }
    retval
}

fn find_and_multiply_two(input: &Vec<i64>) -> i64 {
    let mut a= 0;
    let mut b= 0;
    for (idx, item) in input.iter().enumerate() {
        let mut end = false;
        for i in idx+1..input.len() {
            a = *item;
            b = input[i];
            if (a + b) == 2020 {
                end = true;
                break;
            }
        }
        if end {
            break;
        }
    }
    a * b
}

fn find_and_multiply_three(input: &Vec<i64>) -> i64 {
    let mut a= 0;
    let mut b= 0;
    let mut c= 0;
    for (idx, item) in input.iter().enumerate() {
        let mut end = false;
        for i in idx+1..input.len() {
            for j in i+1..input.len() {
                a = *item;
                b = input[i];
                c = input[j];
                if (a + b + c) == 2020 {
                    end = true;
                    break;
                }
            }
            if end {
                break;
            }
        }
        if end {
            break;
        }
    }
    a * b * c
}

fn main() {
    let input = parse_input();
    println!("pair: {}", find_and_multiply_two(&input));
    println!("triplet: {}", find_and_multiply_three(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_and_multiply_two(&input), 514579);
    }

    #[test]
    fn test_three() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_and_multiply_three(&input), 241861950);
    }
}
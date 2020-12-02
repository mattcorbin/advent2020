use std::fs::File;
use std::io::prelude::*;

struct PasswordReq {
    min_occurrences: usize,
    max_occurrences: usize,
    required_letter: String,
    password: String,
}

impl PasswordReq {
    fn new(min_occurrences: usize, max_occurrences: usize, required_letter: String, password: String) -> PasswordReq {
        PasswordReq {
            min_occurrences,
            max_occurrences,
            required_letter,
            password,
        }
    }

    fn is_valid_count(&self) -> bool {
        let matches: Vec<&str> = self.password.matches(&self.required_letter).collect();
        self.max_occurrences >= matches.len() && matches.len() >= self.min_occurrences
    }

    fn is_valid_pos(&self) -> bool {
        let char_vec: Vec<char> = self.password.chars().collect();
        let first = char_vec[self.min_occurrences-1].to_string();
        let second = char_vec[self.max_occurrences-1].to_string();
        if first != second && (first == self.required_letter || second == self.required_letter) {
            return true;
        }
        false
    }
}

fn parse_input() -> Vec<PasswordReq> {
    let mut retval: Vec<PasswordReq> = Vec::new();
    let mut file = File::open("input.txt").expect("input.txt does not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read contents to string");
    for item in contents.lines() {
        let min_oc: usize;
        let max_oc: usize;
        let c: String;
        let pw: String;
        let v: Vec<&str> = item.split("-").collect();
        min_oc = v[0].parse().expect("failed to parse min value");
        let v: Vec<&str> = v[1].split(": ").collect();
        pw = v[1].to_string();
        let v: Vec<&str> = v[0].split(" ").collect();
        max_oc = v[0].parse().expect("failed to parse max value");
        c = v[1].to_string();
        retval.push(PasswordReq::new(min_oc, max_oc, c, pw));
    }
    retval
}

fn main() {
    let input = parse_input();
    let mut count_resp = 0;
    let mut pos_resp = 0;
    for item in input.into_iter() {
        if item.is_valid_count() {
            count_resp += 1;
        }
        if item.is_valid_pos() {
            pos_resp += 1
        }
    }
    println!("valid count passwords: {}", count_resp);
    println!("valid pos passwords: {}", pos_resp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_count() {
        let valid = PasswordReq {
            min_occurrences: 1,
            max_occurrences: 3,
            required_letter: "a".to_string(),
            password: "abcde".to_string(),
        };
        assert_eq!(valid.is_valid_count(), true);
        let invalid = PasswordReq {
            min_occurrences: 2,
            max_occurrences: 9,
            required_letter: "c".to_string(),
            password: "ccccccccc".to_string(),
        };
        assert_eq!(valid.is_valid_count(), true);
    }

    #[test]
    fn test_invalid_count() {
        let invalid = PasswordReq {
            min_occurrences: 1,
            max_occurrences: 3,
            required_letter: "b".to_string(),
            password: "cdefg".to_string(),
        };
        assert_eq!(invalid.is_valid_count(), false);
        let invalid = PasswordReq {
            min_occurrences: 1,
            max_occurrences: 3,
            required_letter: "b".to_string(),
            password: "".to_string(),
        };
        assert_eq!(invalid.is_valid_count(), false);
    }

    #[test]
    fn test_valid_pos() {
        let valid = PasswordReq {
            min_occurrences: 1,
            max_occurrences: 3,
            required_letter: "a".to_string(),
            password: "abcde".to_string(),
        };
        assert_eq!(valid.is_valid_pos(), true);
    }

    #[test]
    fn test_invalid_pos() {
        let invalid = PasswordReq {
            min_occurrences: 1,
            max_occurrences: 3,
            required_letter: "b".to_string(),
            password: "cdefg".to_string(),
        };
        assert_eq!(invalid.is_valid_pos(), false);
        let invalid = PasswordReq {
            min_occurrences: 1,
            max_occurrences: 3,
            required_letter: "b".to_string(),
            password: "ccccccccc".to_string(),
        };
        assert_eq!(invalid.is_valid_pos(), false);
    }
}
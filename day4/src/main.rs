use std::fs::File;
use std::io::prelude::*;

fn parse_year(min: usize, max: usize, input: &str) -> Result<usize, &str> {
    if input.len() == 4 {
        return match input.parse() {
            Ok(v) => {
                if v >= min && v <= max {
                    return Ok(v);
                }
                Err("year out of bounds")
            },
            Err(_) => { Err("failed to parse int") },
        }
    }
    Err("year incorrect length")
}

struct Height {
    scalar: usize,
    unit: String,
}

impl Height {
    fn parse(input: &str) -> Result<Height, &str> {
        if input.contains("in") {
            return match input.replace("in", "").parse() {
                Ok(v) => {
                    if v >= 59 && v <= 76 {
                        return Ok(Height {
                            scalar: v,
                            unit: "in".to_string()
                        });
                    }
                    Err("height out of range")
                },
                Err(_) => { Err("failed to parse int") },
            }
        } else if input.contains("cm") {
            return match input.replace("cm", "").parse() {
                Ok(v) => {
                    if v >= 150 && v <= 193 {
                        return Ok(Height {
                            scalar: v,
                            unit: "cm".to_string()
                        });
                    }
                    Err("height out of range")
                },
                Err(_) => { Err("failed to parse int") },
            }
        }
        Err("invalid height - no unit")
    }
}

fn parse_hair_colour(input: &str) -> Result<String, &str> {
    if input.len() == 7 {
        let chars: Vec<char> = input.chars().collect();
        if chars[0] != '#' {
            return Err("invalid colour format");
        }
        for i in 1..7 {
            if !chars[i].is_digit(16) {
                return Err("invalid colour format");
            }
        }
        return Ok(input.to_string());
    }
    Err("invalid hair colour length")
}


fn parse_eye_colour(input: &str) -> Result<String, &str> {
    let eye_colours: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if eye_colours.iter().any(|e| input == *e) {
        return Ok(input.to_string());
    }
    Err("invalid eye colour")
}

fn parse_passport_id(input: &str) -> Result<String, &str> {
    if input.len() == 9 {
        return match input.parse::<usize>() {
            Ok(_) => Ok(input.to_string()) ,
            Err(_) => Err("failed to parse int"),
        }
    }
    Err("Invalid passport id length")
}

struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<Height>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_colour: None,
            eye_colour: None,
            passport_id: None,
            country_id: None
        }
    }

    fn is_valid(&self) -> bool {
       self.birth_year.is_some()
           && self.issue_year.is_some()
           && self.expiration_year.is_some()
           && self.height.is_some()
           && self.hair_colour.is_some()
           && self.eye_colour.is_some()
           && self.passport_id.is_some()
    }
}

fn parse_input() -> Vec<Passport> {
    let mut file = File::open("input.txt").expect("input.txt does not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read contents to string");
    let mut passports: Vec<Passport> = Vec::new();
    let mut current_passport = Passport::new();
    for line in contents.lines() {
        if line.len() < 3 {
            passports.push(current_passport);
            current_passport = Passport::new();
        } else {
            for item in line.split(" ") {
                let pairs: Vec<&str> = item.split(":").collect();
                match pairs[0] {
                    "byr" => {
                        current_passport.birth_year = match parse_year(1920, 2002, pairs[1]) {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        }
                    },
                    "iyr" => {
                        current_passport.issue_year = match parse_year(2010, 2020, pairs[1]) {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        }
                    },
                    "eyr" => {
                        current_passport.expiration_year = match parse_year(2020, 2030, pairs[1]) {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        }
                    },
                    "hgt" => {
                        current_passport.height = match Height::parse(pairs[1]) {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        }
                    },
                    "hcl" => {
                        current_passport.hair_colour = match parse_hair_colour(pairs[1]) {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        }
                    },
                    "ecl" => {
                        current_passport.eye_colour = match parse_eye_colour(pairs[1]) {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        }
                    },
                    "pid" => {
                        current_passport.passport_id = match parse_passport_id(pairs[1]) {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        }
                    },
                    "cid" => current_passport.country_id = Some(pairs[1].to_string()),
                    _ => println!("unrecognized field: {}:{}", pairs[0], pairs[1]),
                }
            }
        }
    }
    passports
}

fn main() {
    let passports = parse_input();
    let mut valid_passports = 0;
    for passport in passports {
        if passport.is_valid() {
            valid_passports += 1;
        }
    }
    println!("valid passports: {}", valid_passports);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let passport = Passport{
            birth_year: Some(1920),
            issue_year: Some(1),
            expiration_year:Some(1),
            height: Some(Height),
            hair_colour: Some("a".to_string()),
            eye_colour: Some("a".to_string()),
            passport_id: Some("a".to_string()),
            country_id: None
        };
        assert_eq!(passport.is_valid(), true);
        let passport = Passport{
            birth_year: None,
            issue_year: Some(1),
            expiration_year: Some(1),
            height: None,
            hair_colour: Some("a".to_string()),
            eye_colour: Some("a".to_string()),
            passport_id: Some("a".to_string()),
            country_id: None
        };
        assert_eq!(passport.is_valid(), false);
    }

}
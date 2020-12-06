use crate::common;
use std::string::ParseError;

pub struct Input {
    lower_bound: i32,
    upper_bound: i32,
    character: char,
    password: String
}

impl std::str::FromStr for Input {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split the whole line
        let split_result: Vec<&str> = s.split_whitespace().collect();
        
        // split the lower and upper bound
        let bound_split: Vec<&str> = split_result[0].split('-').collect();
        let lower_bound = bound_split[0].parse::<i32>().unwrap();
        let upper_bound = bound_split[1].parse::<i32>().unwrap();

        // get the character
        let character = split_result[1].as_bytes()[0] as char;

        // get the password
        let password = split_result[2].parse::<String>().unwrap();

        let input = Input { lower_bound, upper_bound, character, password };

        Ok(input)
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Input> {
    common::input_vec(input)
}

#[aoc(day2, part1)]
pub fn solve_part_01(input: &[Input]) -> usize {
    let mut valid = 0;
    
    for p in input{
        let matches = p.password.matches(p.character).count() as i32;

        if matches >= p.lower_bound && matches <= p.upper_bound {
            valid += 1;
        }
    }

    valid
}

#[aoc(day2, part2)]
pub fn solve_part_02(input: &[Input]) -> usize {
    let mut valid = 0;
    
    for p in input {
        let byte_pass = p.password.as_bytes();
        let char_lower = byte_pass[(p.lower_bound - 1) as usize] as char;
        let char_upper = byte_pass[(p.upper_bound - 1) as usize] as char;
        if (char_lower == p.character || char_upper == p.character) 
            && char_lower != char_upper {
            valid += 1;
        }
    }

    valid
}
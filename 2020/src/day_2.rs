use std::fs::File;
use std::io::Read;

struct Input {
    lower_bound: i32,
    upper_bound: i32,
    character: char,
    password: String
}

fn read_input() -> Vec<Input> {
    let mut file = File::open("input/day_2.txt").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut result: Vec<Input> = Vec::new();

    for s in contents.lines() {
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
        result.push(input);
    }

    result
}

fn question_one() {
    let data = read_input();
    let mut valid = 0;
    
    for p in data {
        let matches = p.password.matches(p.character).count() as i32;

        if matches >= p.lower_bound && matches <= p.upper_bound {
            valid += 1;
        }
    }

    println!("Question 1: Valid passwords: {}", valid);
}

fn question_two() {
    let data = read_input();
    let mut valid = 0;
    
    for p in data {
        let byte_pass = p.password.as_bytes();
        let char_lower = byte_pass[(p.lower_bound - 1) as usize] as char;
        let char_upper = byte_pass[(p.upper_bound - 1) as usize] as char;
        if (char_lower == p.character || char_upper == p.character) 
            && char_lower != char_upper {
            valid += 1;
        }
    }

    println!("Question 2: Valid passwords: {}", valid);
}

fn main() {
    question_one();
    question_two();
}
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn question_one() {
    let data = read_input();
    let mut set: HashSet<u32> = HashSet::new();
    let target = 2020;
    let mut result = 0;

    for num in data {
        if set.contains(&(target - num)) {
            result = num * (target - num);
            break;
        } else {
            set.insert(num);
        }
    }

    println!("Question one: Result: {}", result)
}

fn question_two() {
    let data = read_input();
    let target: u32 = 2020;
    let mut result = 0;

    for x in &data {
        for y in &data {
            for z in &data {
                if x + y + z == target {
                    result = x * y * z;
                }
            }
        }
    }

    println!("Question two: Result: {}", result)
}

fn read_input() -> Vec<u32> {
    let mut file = File::open("input/day_1.txt").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut v: Vec<u32> = Vec::new();

    for s in contents.lines() {
        v.push(s.parse::<u32>().unwrap());
    }

    v
}

fn main() {
    question_one();
    question_two();
}
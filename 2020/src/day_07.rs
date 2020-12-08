use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref COLOR_RE: Regex = Regex::new(r"^(.+?) bags contain").unwrap();
    static ref BAG_RE: Regex = Regex::new(r"(\d+) (.+?) bags?[,.]").unwrap();
}

#[derive(Debug)]
pub struct Bag {
    color: String,
    bags: Vec<(u32, String)>
}

impl Bag {
    fn new(s: &str) -> Self {
        let color = COLOR_RE
            .captures_iter(s)
            .map(|c| c[1].to_string())
            .collect();

        let bags = BAG_RE
            .captures_iter(s)
            .map(|b| (b[1].to_string().parse().unwrap(), b[2].to_string()))
            .collect();

        Bag { color, bags }
    }
}

#[aoc_generator(day7)]
pub fn input_generator_part_01(input: &str) -> HashMap<String, Bag> {
    let data: Vec<&str> = input.lines().collect();
    let bags: Vec<Bag> = data.into_iter().map(|l| Bag::new(l)).collect(); 
    make_map(bags)
}

fn make_map(bags: Vec<Bag>) -> HashMap<String, Bag> {
    let mut map: HashMap<String, Bag> = HashMap::new();

    for bag in bags {
        let name = String::from(&bag.color);
        map.insert(name, bag);
    }

    map
}

fn search_bag(input: &Bag, all_bags: &HashMap<String, Bag>, target_color: &String) -> bool {
    if input.bags.is_empty() {
        return false;
    }

    for (_, inner_color) in &input.bags {
        match inner_color == target_color {
            true => return true,
            _ => match search_bag(all_bags.get(inner_color).unwrap(), all_bags, &target_color) {
                true => return true,
                _ => continue
            }
        }
    }

    false
}

fn search_bags(input: &HashMap<String, Bag>, all_bags: &HashMap<String, Bag>, target_bags: &mut HashSet<String>, target_color: &String) {
    for (key, bag) in input {
        if search_bag(bag, all_bags, target_color) {
            target_bags.insert(key.clone());
        }
    }
}

#[aoc(day7, part1)]
pub fn solve_part_01(input: &HashMap<String, Bag>) -> usize {
    let mut gold_bags: HashSet<String> = HashSet::new();
    search_bags(input, &input, &mut gold_bags, &"shiny gold".to_string());
    gold_bags.len()
}

fn count_bags(input: &HashMap<String, Bag>, target_color: &String) -> usize {
    let bag = input.get(target_color).unwrap();

    let mut total: usize = 0;

    for (quantity, inner_color) in &bag.bags {
        total += *quantity as usize;
        total += *quantity as usize * count_bags(input, inner_color);
    }

    total
}

#[aoc(day7, part2)]
pub fn solve_part_02(input: &HashMap<String, Bag>) -> usize {
    count_bags(input, &"shiny gold".to_string())
}
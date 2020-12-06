use std::collections::{HashSet, HashMap};

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[aoc_generator(day6, part1)]
pub fn input_generator_part_01(input: &str) -> Vec<String> {
    let data: Vec<&str> = input.split("\n\n").collect();
    data.into_iter().map(|l| remove_whitespace(l)).map(|l| String::from(l)).collect()
}

#[aoc_generator(day6, part2)]
pub fn input_generator_part_02(input: &str) -> Vec<String> {
    let data: Vec<&str> = input.split("\n\n").collect();
    data.into_iter().map(|l| String::from(l)).collect()
}

#[aoc(day6, part1)]
pub fn solve_part_01(input: &[String]) -> usize {
    input
    .iter()
    .map(|group| group.chars().collect::<HashSet<_>>().len())
    .sum()
}

#[aoc(day6, part2)]
pub fn solve_part_02(input: &[String]) -> usize {
    let mut answer_sum = 0;

    for group in input {
        let mut group_answers: HashMap<u32, HashSet<char>> = HashMap::new();

        let people: Vec<String> = group.split("\n").
                                        into_iter().
                                        map(|l| String::from(l)).
                                        collect();

        for (i, person) in people.iter().enumerate() {
            let mut answers: HashSet<char> = HashSet::new();
            if (i == 0) {
                person.chars().map(|c| answers.insert(c));
            } else {
                for c in person.chars() {
                    if group_answers.get(&((i as u32) - 1)).unwrap().contains(&c) {
                        answers.insert(c);
                    }
                }
            }
            group_answers.insert(i as u32, answers);
        }

        answer_sum += group_answers.get(&((people.len() - 1) as u32)).unwrap().len();
        
    }

    answer_sum
}
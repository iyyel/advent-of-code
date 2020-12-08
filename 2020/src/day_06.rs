use std::collections::HashSet;

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
        let people: Vec<String> = group.split("\n").
                                        into_iter().
                                        map(|l| String::from(l)).
                                        collect();

        let mut person_zero_answers: HashSet<char> = HashSet::new();

        for c in people[0].chars() {
            person_zero_answers.insert(c);
        }

        for (i, person) in people.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let clone_map = person_zero_answers.clone();
           
            for c in clone_map {
                if !person.chars().any(|x| x == c) {
                    person_zero_answers.remove(&c);
                }
            }
        }

       answer_sum += person_zero_answers.len();
    }

    answer_sum
}
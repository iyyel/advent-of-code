use crate::common;

fn slope_finder(input: &[String], rs: &usize, cs: &usize) -> u32 {
    let rows = input.len();
    let mut trees = 0;
    let (mut row, mut col) = (0, 0);

    while row < rows {
        let cols = input[row].len();

        if input[row].as_bytes()[col % cols] == b'#' {
            trees += 1
        }

        row += rs;
        col += cs;
    }

    trees
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    common::input_vec(input)
}

#[aoc(day3, part1)]
pub fn solve_part_01(input: &[String]) -> u32 {
    slope_finder(input, &1, &3)
}

#[aoc(day3, part2)]
pub fn solve_part_02(input: &[String]) -> u32 {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().fold(1, |acc, (rows, cols)| acc * slope_finder(input, rows, cols))
}
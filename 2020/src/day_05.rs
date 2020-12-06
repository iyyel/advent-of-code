use crate::common;
use std::ops::RangeInclusive;

fn find_seat(seat: &str) -> u32 {
    let mut row = 0..=127;
    let mut col = 0..=7;
 
    for c in seat.chars() {
        match c {
            'F' => row = lower_half(&row),
            'B' => row = upper_half(&row),
            'R' => col = upper_half(&col),
            'L' => col = lower_half(&col),
            _ => continue
        }
    }

    row.start() * 8 + col.start()
}

fn half_range(range: &RangeInclusive<u32>) -> u32 {
    ((*range.end() as f32 - *range.start() as f32) as f32 / 2_f32).round() as u32
}

fn upper_half(range: &RangeInclusive<u32>) -> RangeInclusive<u32> {
    *range.start() + half_range(range)..=*range.end()
}

fn lower_half(range: &RangeInclusive<u32>) -> RangeInclusive<u32> {
    *range.start()..=*range.end() - half_range(range)
}

fn get_seats(input: &[String]) -> Vec<u32> {
    input.iter().map(|seat| find_seat(seat)).collect()
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    common::input_vec(input)
}

#[aoc(day5, part1)]
pub fn solve_part_01(input: &[String]) -> u32 {
    get_seats(input).into_iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part_02(input: &[String]) -> u32 {
    let mut seats: Vec<u32> = get_seats(input);
    seats.sort();

    let mut my_seat = 0;

    for (i, seat) in seats.iter().enumerate() {
        if seat + 1 != seats[i + 1] {
            my_seat = seat + 1;
            break;
        }
    }

    my_seat
}
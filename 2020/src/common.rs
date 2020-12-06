use std::collections::HashSet;
use std::{cmp, fmt::Debug, hash, str::FromStr};

fn clean_input(input: &str) -> impl Iterator<Item = &str> {
    input.lines().map(|l| l.trim()).filter(|l| !l.is_empty())
}

pub fn input_vec<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    clean_input(input).map(|l| l.parse().unwrap()).collect()
}

pub fn input_hashset<T>(input: &str) -> HashSet<T>
where
    T: FromStr + cmp::Eq + hash::Hash,
    <T as FromStr>::Err: Debug,
{
    clean_input(input).map(|l| l.parse().unwrap()).collect()
}
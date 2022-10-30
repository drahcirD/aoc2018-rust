use itertools::Itertools;
use std::{collections::HashMap, iter::zip};

pub fn part_one(input: &str) -> i64 {
    let mut twos = 0;
    let mut threes = 0;
    for id in input.lines() {
        let mut chars = HashMap::new();
        id.chars().for_each(|f| *chars.entry(f).or_insert(0) += 1);
        if chars.values().any(|f| *f == 2) {
            twos += 1;
        }
        if chars.values().any(|f| *f == 3) {
            threes += 1
        }
    }
    twos * threes
}

fn diff_is_one(a: &str, b: &str) -> bool {
    let mut diff = 0;
    for (x, y) in zip(a.chars(), b.chars()) {
        diff += (x != y) as u8;
        if diff > 1 {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> String {
    let (a,b) = input
        .lines()
        .tuple_combinations()
        .filter(|(a, b)| diff_is_one(a, b))
        .exactly_one()
        .unwrap();
    
    zip(a.chars(), b.chars())
        .filter(|(x, y)| x == y)
        .map(|(x, _y)| x)
        .collect()
}

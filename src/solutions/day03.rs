use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::i32;
use std::str::FromStr;

use regex::Regex;

struct Claim {
    id: i16,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl FromStr for Claim {
    type Err = std::num::ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(?P<id>[0-9]+)\s@\s(?P<x>[0-9]+),(?P<y>[0-9]+):\s(?P<width>[0-9]+)x(?P<height>[0-9]+)").unwrap();
        }
        let cap = match RE.captures(&input) {
            Some(data) => data,
            None => panic!("Regex capture problem"),
        };
        Ok(Claim {
            id: cap.name("id").unwrap().as_str().parse::<i16>()?,
            x: cap.name("x").unwrap().as_str().parse::<i32>()?,
            y: cap.name("y").unwrap().as_str().parse::<i32>()?,
            width: cap.name("width").unwrap().as_str().parse::<i32>()?,
            height: cap.name("height").unwrap().as_str().parse::<i32>()?,
        })
    }
}

pub fn part_one(input: &str) -> usize {
    let mut claims = HashMap::<(i32, i32), i32>::new();
    for claim in input.lines().map(|x| x.parse::<Claim>().unwrap()) {
        for i in claim.x..claim.x + claim.width {
            for j in claim.y..claim.y + claim.height {
                *claims.entry((i, j)).or_insert(0) += 1;
            }
        }
    }
    claims.values().filter(|v| **v >= 2).count()
}

pub fn part_two(input: &str) -> i16 {
    let claims: Vec<Claim> = input.lines().map(|x| x.parse::<Claim>().unwrap()).collect();
    let mut patterns = HashMap::<(i32, i32), i16>::new();
    let mut ids = HashSet::<i16>::new();
    for claim in &claims {
        ids.insert(claim.id);
        for i in claim.x..claim.x + claim.width {
            for j in claim.y..claim.y + claim.height {
                if *patterns.entry((i, j)).or_insert(claim.id) != claim.id {
                    ids.remove(&claim.id);
                    ids.remove(patterns.get(&(i, j)).unwrap());
                }
            }
        }
    }
    ids.iter().next().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use rust::read_file;

    #[test]
    fn p1() {
        assert_eq!(
            crate::solutions::day03::part_one(&read_file("examples", 3)),
            4
        );
    }

    #[test]
    fn p2() {
        assert_eq!(
            crate::solutions::day03::part_two(&read_file("examples", 3)),
            3
        );
    }
}

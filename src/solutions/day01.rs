use std::collections::HashSet;

pub fn part_one(input: &str) -> i64 {
    input.lines().map(|x| x.parse::<i64>().unwrap()).sum()
}

pub fn part_two(input: &str) -> i64 {
    let mut cur_frequency: i64 = 0;
    let mut frequencies = HashSet::from([0]);
    for change in input.lines().map(|x| x.parse::<i64>().unwrap()).cycle() {
        cur_frequency += change;
        if frequencies.contains(&cur_frequency) {
            return cur_frequency;
        }
        frequencies.insert(cur_frequency);
    }
    unreachable!();
}

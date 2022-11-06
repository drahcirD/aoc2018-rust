use std::collections::HashMap;

use itertools::Itertools;

/*
input: ["[1518-11-22", "23:48]", "Guard", "#797", "begins", "shift"]
output: 23,48
*/
fn get_time(parts: &[&str]) -> (u32, u32) {
    let hour = (parts[1][..2]).parse::<u32>().unwrap();
    let min = (parts[1][3..5]).parse::<u32>().unwrap();
    (hour, min)
}

fn parse_guards(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut guards: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lines = input.lines().collect::<Vec<&str>>();
    lines.sort();
    let mut i = 0;
    while i < lines.len() {
        let parts = lines[i].split_whitespace().collect::<Vec<&str>>();
        let guard_id = match (parts[3][1..]).parse::<u32>() {
            Ok(id) => id,
            Err(error) => panic!(
                "Problem parsing integer: {:?} from {}",
                error,
                &parts[3][1..]
            ),
        };
        i += 1;
        let mut parts2 = lines[i].split(' ').collect::<Vec<&str>>();
        while parts2[2] == "falls" {
            let (_start_hour, start_min) = get_time(&parts2);
            i += 1;
            parts2 = lines[i].split(' ').collect::<Vec<&str>>();
            assert!(parts2[2] == "wakes");
            let (_end_hour, end_min) = get_time(&parts2);
            guards
                .entry(guard_id)
                .or_default()
                .extend(start_min..end_min);
            i += 1;
            if i >= lines.len() {
                break;
            }
            parts2 = lines[i].split(' ').collect::<Vec<&str>>();
        }
    }
    guards
}

pub fn part_one(input: &str) -> u32 {
    let guards = parse_guards(input);
    let sleepiest = guards
        .iter()
        .max_by(|v1, v2| v1.1.iter().len().cmp(&v2.1.iter().len()))
        .unwrap()
        .0;
    let most_common = guards
        .get(sleepiest)
        .unwrap()
        .iter()
        .counts()
        .into_iter()
        .max_by(|v1, v2| v1.1.cmp(&v2.1))
        .unwrap()
        .0;
    sleepiest * most_common
}

pub fn part_two(input: &str) -> u32 {
    let guards = parse_guards(input);
    let mut max = 0;
    let mut max_guard = 0;
    for (guard, sleeps) in guards.into_iter() {
        let most_common = sleeps
            .into_iter()
            .counts()
            .into_iter()
            .max_by(|v1, v2| v1.1.cmp(&v2.1))
            .unwrap()
            .0;
        if most_common > max {
            max = most_common;
            max_guard = guard;
        }
    }
    max * max_guard
}

#[cfg(test)]
mod tests {
    use rust::read_file;

    #[test]
    fn p1() {
        assert_eq!(
            crate::solutions::day04::part_one(&read_file("examples", 4)),
            240
        );
    }

    #[test]
    fn p2() {
        assert_eq!(
            crate::solutions::day04::part_two(&read_file("examples", 4)),
            4455
        );
    }
}

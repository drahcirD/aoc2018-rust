use rustc_hash::FxHashSet;

fn get_index(i: usize, removed: &FxHashSet<usize>) -> usize {
    let mut j = i;
    while removed.contains(&j) {
        j += 1;
    }
    j
}

pub fn part_one(input: &str) -> usize {
    let mut removed_indices = FxHashSet::<usize>::default();
    let chars: Vec<char> = input.chars().collect();
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..chars.len() - 1 {
            if removed_indices.contains(&i) {
                continue;
            }
            let next_index = get_index(i + 1, &removed_indices);
            if next_index >= chars.len() {
                continue;
            }
            if chars[i] != chars[next_index]
                && chars[i].to_ascii_lowercase() == chars[next_index].to_ascii_lowercase()
            {
                removed_indices.insert(i);
                removed_indices.insert(next_index);
                changed = true;
            }
        }
    }
    chars.len() - removed_indices.len()
}

pub fn part_two(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut min = usize::MAX;
    for unit in ['a', 'b', 'c', 'd'] {
        let mut removed_indices = FxHashSet::<usize>::from_iter(
            chars
                .iter()
                .enumerate()
                .filter(|(_, &v)| v.to_ascii_lowercase() == unit)
                .map(|(index, _)| index),
        );
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..chars.len() - 1 {
                if removed_indices.contains(&i) {
                    continue;
                }
                let next_index = get_index(i + 1, &removed_indices);
                if next_index >= chars.len() {
                    continue;
                }
                if chars[i] != chars[next_index]
                    && chars[i].to_ascii_lowercase() == chars[next_index].to_ascii_lowercase()
                {
                    removed_indices.insert(i);
                    removed_indices.insert(next_index);
                    changed = true;
                }
            }
        }
        let cur_len = chars.len() - removed_indices.len();
        if min >= cur_len {
            min = cur_len;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use rust::read_file;

    #[test]
    fn p1() {
        assert_eq!(
            crate::solutions::day05::part_one(&read_file("examples", 5)),
            10
        );
    }

    #[test]
    fn p2() {
        assert_eq!(
            crate::solutions::day05::part_two(&read_file("examples", 5)),
            4
        );
    }
}

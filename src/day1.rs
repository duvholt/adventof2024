use std::collections::{hash_map::Entry, HashMap};

fn parse_input(contents: String) -> (Vec<u64>, Vec<u64>) {
    let lines: Vec<_> = contents
        .lines()
        .map(|s| {
            let mut a = s.split_whitespace();
            let first = a.next().unwrap().parse().unwrap();
            let second = a.next().unwrap().parse().unwrap();
            (first, second)
        })
        .collect();
    let first: Vec<_> = lines.iter().map(|(l, _)| *l).collect();
    let second: Vec<_> = lines.into_iter().map(|(_, l)| l).collect();
    (first, second)
}

pub fn part1(contents: String) -> String {
    let (mut first, mut second) = parse_input(contents);
    first.sort();
    second.sort();

    let s: u64 = first
        .into_iter()
        .zip(second)
        .map(|(f, s)| f.abs_diff(s))
        .sum();
    s.to_string()
}

pub fn part2(contents: String) -> String {
    let (first, second) = parse_input(contents);

    let mut second_map = HashMap::new();
    for s in second {
        match second_map.entry(s) {
            Entry::Occupied(occupied_entry) => *(occupied_entry.into_mut()) += 1,
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        }
    }

    let s: u64 = first
        .into_iter()
        .map(|f| f * second_map.get(&f).unwrap_or(&0))
        .sum();
    s.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/1/real.txt").unwrap()),
            "2580760"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/1/real.txt").unwrap()),
            "25358365"
        );
    }

    #[test]
    fn test_part1_bigboy() {
        assert_eq!(
            part1(fs::read_to_string("./input/1/bigboy.txt").unwrap()),
            "70030075280"
        );
    }

    #[test]
    fn test_part2_bigboy() {
        assert_eq!(
            part2(fs::read_to_string("./input/1/bigboy.txt").unwrap()),
            "112445724586901"
        );
    }
}

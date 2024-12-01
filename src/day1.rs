use rustc_hash::FxBuildHasher;
use std::collections::{hash_map::Entry, HashMap};

fn parse_input(contents: String) -> (Vec<u64>, Vec<u64>) {
    let lines: Vec<&str> = contents.split_ascii_whitespace().collect();
    let mut first = Vec::with_capacity(lines.len());
    let mut second = Vec::with_capacity(lines.len());

    for chunk in lines.chunks(2) {
        let f: u64 = chunk[0].parse().unwrap();
        let s: u64 = chunk[1].parse().unwrap();
        first.push(f);
        second.push(s);
    }
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

    let mut second_map = HashMap::with_capacity_and_hasher(second.len(), FxBuildHasher);
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
        .flat_map(|f| second_map.get(&f).map(|s| f * s))
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

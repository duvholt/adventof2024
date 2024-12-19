use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    vec,
};

use rustc_hash::{FxHashMap, FxHashSet};

pub fn part1(contents: String) -> String {
    let mut parts = contents.split("\n\n");
    let patterns: Vec<&str> = parts.next().unwrap().split(", ").collect();

    let towel_designs: Vec<_> = parts.next().unwrap().lines().collect();

    let mut possible = 0;
    for towel_design in towel_designs {
        let mut stack = Vec::new();
        stack.push(towel_design.to_string());
        while let Some(wip_design) = stack.pop() {
            if wip_design.is_empty() {
                possible += 1;
                break;
            }
            for pattern in patterns.iter() {
                if let Some(stripped) = wip_design.strip_prefix(pattern) {
                    stack.push(stripped.to_string());
                }
            }
        }
    }

    possible.to_string()
}

pub fn part2(contents: String) -> String {
    let mut parts = contents.split("\n\n");
    let patterns: Vec<&str> = parts.next().unwrap().split(", ").collect();

    let towel_designs: Vec<_> = parts.next().unwrap().lines().collect();

    let mut possible = 0;
    for towel_design in towel_designs {
        let mut stack: VecDeque<(String, Vec<String>)> = VecDeque::new();
        stack.push_back((towel_design.to_string(), vec![]));
        let mut visited = FxHashSet::default();
        let mut count_map: FxHashMap<String, Vec<String>> = FxHashMap::default();

        let mut goal = false;

        while let Some((wip_design, path)) = stack.pop_front() {
            if wip_design.is_empty() {
                goal = true;
                continue;
            }
            for pattern in patterns.iter() {
                if let Some(stripped) = wip_design.strip_prefix(pattern) {
                    match count_map.entry(stripped.to_string()) {
                        Entry::Occupied(occupied_entry) => {
                            let a = occupied_entry.into_mut();
                            a.push(wip_design.clone());
                        }
                        Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(vec![wip_design.clone()]);
                            let mut new = path.clone();
                            new.push(stripped.to_string());
                            stack.push_back((stripped.to_string(), new));
                        }
                    }
                }
            }
            visited.insert(path);
        }
        if goal {
            let s = count_paths(towel_design, count_map);
            possible += s;
        }
    }

    possible.to_string()
}

fn count_paths(
    towel_design: &str,
    count_map: std::collections::HashMap<String, Vec<String>, rustc_hash::FxBuildHasher>,
) -> usize {
    // let sink = towel_design;
    // let source = "";
    let mut reversed_count_map: FxHashMap<String, Vec<String>> = FxHashMap::default();
    for (key, values) in count_map.iter() {
        for value in values {
            let entry = reversed_count_map.entry(value.to_string()).or_default();
            entry.push(key.to_string());
        }
    }

    let sink = "";
    let source = towel_design;

    let l = topological_sort(&reversed_count_map, source);

    let mut part_count: FxHashMap<&str, usize> = FxHashMap::default();

    for part in l {
        let part_value = part_count.get(part).cloned().unwrap_or(1);

        let parts = reversed_count_map.get(part);
        if parts.is_none() {
            continue;
        }
        for p in parts.unwrap() {
            match part_count.entry(p) {
                Entry::Occupied(occupied_entry) => {
                    *occupied_entry.into_mut() += part_value;
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(part_value);
                }
            }
        }
    }

    part_count.get(sink).cloned().unwrap()
}

fn topological_sort<'a>(
    reversed_count_map: &'a HashMap<String, Vec<String>, rustc_hash::FxBuildHasher>,
    source: &'a str,
) -> Vec<&'a str> {
    let mut fre: HashMap<String, usize> = HashMap::default();

    for (_key, value) in reversed_count_map.iter() {
        for v in value {
            let entry = fre.entry(v.to_string()).or_default();
            *entry += 1;
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(source);

    let mut nodes = Vec::new();
    while let Some(node) = queue.pop_front() {
        nodes.push(node);
        let parts = reversed_count_map.get(node);
        if parts.is_none() {
            continue;
        }
        for p in parts.unwrap() {
            let edges = fre.get_mut(p).unwrap();
            *edges -= 1;
            if *edges == 0 {
                queue.push_back(p);
            }
        }
    }
    nodes
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/19/real.txt").unwrap()),
            "238"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/19/real.txt").unwrap()),
            "635018909726691"
        );
    }
}

use std::{
    collections::{hash_map::Entry, VecDeque},
    default, vec,
};

use rustc_hash::{FxHashMap, FxHashSet, FxHasher};

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
    // dbg!(patterns, towel_designs);

    let mut possible = 0;
    for towel_design in towel_designs {
        dbg!(towel_design);
        let mut stack: VecDeque<(String, Vec<String>)> = VecDeque::new();
        stack.push_back((towel_design.to_string(), vec![]));
        let mut visited = FxHashSet::default();
        let mut count_map: FxHashMap<String, Vec<String>> = FxHashMap::default();

        // let mut paths = vec![];
        let mut goal = false;

        while let Some((wip_design, path)) = stack.pop_front() {
            if wip_design.is_empty() {
                // dbg!(possible, dbg!(path));
                // paths.push(path);
                goal = true;
                continue;
            }
            // if visited.contains(&path) {
            //     continue;
            // }
            for pattern in patterns.iter() {
                if let Some(stripped) = wip_design.strip_prefix(pattern) {
                    match count_map.entry(stripped.to_string()) {
                        Entry::Occupied(occupied_entry) => {
                            let a = occupied_entry.into_mut();
                            a.push(wip_design.clone());
                            // dbg!(&pattern, &stripped, a);
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

        // dbg!(paths);

        // dbg!(&towel_design, &paths);
        dbg!(&count_map);
        if goal {
            let s = count_goals3(towel_design, count_map, goal);
            possible += s;
        }
    }

    possible.to_string()
}

// fn count_goals2(
//     towel_design: &str,
//     count_map: std::collections::HashMap<String, Vec<String>, rustc_hash::FxBuildHasher>,
//     goal: bool,
// ) -> i32 {
//     let mut reversed_count_map: FxHashMap<String, Vec<String>> = FxHashMap::default();
//     for (key, values) in count_map.iter() {
//         for value in values {
//             let entry = reversed_count_map.entry(value.to_string()).or_default();
//             entry.push(key.to_string());
//         }
//     }

//     let mut s = 0;

//     let mut stack = Vec::new();
//     stack.push((towel_design, 1));

//     dbg!(&reversed_count_map);

//     while let Some((part, count)) = stack.pop() {
//         dbg!(&stack);
//         if part.is_empty() {
//             s += count;
//         } else if let Some(nodes) = reversed_count_map.get(part) {
//             for p in nodes {
//                 if p.is_empty() {
//                     s += 1;
//                 } else {
//                     dbg!(&p);
//                     stack.push((p, count));
//                 }
//             }
//         }
//     }
//     s
// }

fn count_goals(
    towel_design: &str,
    count_map: std::collections::HashMap<String, Vec<String>, rustc_hash::FxBuildHasher>,
    goal: bool,
) -> i32 {
    let mut s = 0;
    if goal {
        let mut stack = Vec::new();
        stack.push(("", "".to_string()));
        while let Some((part, substring)) = stack.pop() {
            if part == towel_design {
                s += 1;
            } else {
                for p in count_map.get(part).unwrap() {
                    let mut sub = substring.clone();
                    sub.push_str(p);
                    dbg!(&sub);
                    if towel_design.strip_prefix(&sub).is_some() {
                        stack.push((p, sub));
                    }
                }
            }
        }
    }
    s
}

fn count_goals3(
    towel_design: &str,
    count_map: std::collections::HashMap<String, Vec<String>, rustc_hash::FxBuildHasher>,
    goal: bool,
) -> i32 {
    let mut reversed_count_map: FxHashMap<String, Vec<String>> = FxHashMap::default();
    for (key, values) in count_map.iter() {
        for value in values {
            let entry = reversed_count_map.entry(value.to_string()).or_default();
            entry.push(key.to_string());
        }
    }

    let mut s = 0;

    let mut part_count: FxHashMap<&str, usize> = FxHashMap::default();
    let mut stack = Vec::new();
    stack.push(towel_design);

    while !stack.is_empty() {
        let (index, part) = stack
            .iter()
            .enumerate()
            .min_by(|(i1, s1), (i2, s2)| {
                let c1 = part_count.get(*s1).unwrap_or(&usize::MAX);
                let c2 = part_count.get(*s2).unwrap_or(&usize::MAX);
                if c1 == c2 {
                    i1.cmp(i2)
                } else {
                    c1.cmp(c2)
                }
            })
            .unwrap();
        let part = part.to_string();
        dbg!(&part_count, index, &part, &stack);
        stack.remove(index);
        // find lowest number in stack?
        if part == "" {
            continue;
        }
        // dbg!(&part_count.get(part.as_str()));
        for p in reversed_count_map.get(&part).unwrap() {
            match part_count.entry(p) {
                Entry::Occupied(occupied_entry) => {
                    dbg!(p);
                    *occupied_entry.into_mut() += 1;
                }
                Entry::Vacant(vacant_entry) => {
                    stack.push(p);
                    vacant_entry.insert(1);
                }
            }
        }
    }
    dbg!(part_count);

    123
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/19/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/19/real.txt").unwrap()),
            "example2"
        );
    }
}

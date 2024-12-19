use std::collections::{hash_map::Entry, VecDeque};

use rustc_hash::FxHashMap;

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

    let mut paths = 0;
    for towel_design in towel_designs {
        let mut queue = Vec::new();
        queue.push(towel_design.to_string());

        // build graph of patterns from full design to empty string
        let mut edges: FxHashMap<String, Vec<String>> = FxHashMap::default();

        while let Some(wip_design) = queue.pop() {
            if wip_design.is_empty() {
                // all patterns found
                continue;
            }
            for pattern in patterns.iter() {
                if let Some(stripped) = wip_design.strip_prefix(pattern) {
                    match edges.entry(stripped.to_string()) {
                        Entry::Occupied(occupied_entry) => {
                            let a = occupied_entry.into_mut();
                            a.push(wip_design.clone());
                        }
                        Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(vec![wip_design.clone()]);
                            // only visit node if it's the first time we see it
                            queue.push(stripped.to_string());
                        }
                    }
                }
            }
        }
        // a solution is only possible if there's an edge to the empty node
        let possible = edges.contains_key("");
        if possible {
            let s = count_paths(towel_design, edges);
            paths += s;
        }
    }

    paths.to_string()
}

fn count_paths(towel_design: &str, edges: FxHashMap<String, Vec<String>>) -> usize {
    // reverse graph
    // probably not necessary but sometimes bugs out if I don't do it in reverse order
    let mut reversed_edges: FxHashMap<String, Vec<String>> = FxHashMap::default();
    for (key, values) in edges.iter() {
        for value in values {
            let entry = reversed_edges.entry(value.to_string()).or_default();
            entry.push(key.to_string());
        }
    }

    let sink = "";
    let source = towel_design;

    // we need to traverse nodes in topological order to count all paths correctly
    let nodes = topological_sort(&reversed_edges, source);

    let mut cumulative_edge_count: FxHashMap<&str, usize> = FxHashMap::default();

    for part in nodes {
        let parts = reversed_edges.get(part);
        if parts.is_none() {
            continue;
        }
        let part_value = cumulative_edge_count.get(part).cloned().unwrap_or(1);
        for p in parts.unwrap() {
            *cumulative_edge_count.entry(p).or_default() += part_value;
        }
    }

    cumulative_edge_count.get(sink).cloned().unwrap()
}

fn topological_sort<'a>(
    reversed_count_map: &'a FxHashMap<String, Vec<String>>,
    source: &'a str,
) -> Vec<&'a str> {
    // Kahn’s algorithm
    let mut fre: FxHashMap<String, usize> = FxHashMap::default();

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

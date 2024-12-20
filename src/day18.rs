use std::{
    collections::{hash_map::Entry, BinaryHeap, HashSet},
    hash::Hash,
};

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Node(Position, u64);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

pub fn part1(contents: String) -> String {
    // let bytes = 12;
    let bytes = 1024;
    let map_grid = parse(contents);

    let map_grid: FxHashSet<Position> = map_grid.into_iter().take(bytes).collect();

    let start = (0, 0);
    // let end = (6, 6);
    let end = (70, 70);

    let node = find_path(start, end, &map_grid).unwrap();

    // subtract start
    (node.len() - 1).to_string()
}

pub fn part2(contents: String) -> String {
    let start = (0, 0);
    // let end = (6, 6);
    let end = (70, 70);

    // let mut bytes = 12;
    let mut bytes = 1024;

    let map_grid = parse(contents);

    // bruteforce ftw
    loop {
        let map_grid_hash: FxHashSet<Position> = map_grid.iter().cloned().take(bytes).collect();
        if find_path(start, end, &map_grid_hash).is_none() {
            let p = map_grid[bytes - 1];
            return format!("{},{}", p.0, p.1);
        }
        bytes += 1;
    }
}

fn traverse_path(path_map: &FxHashMap<Position, Position>, start: &Position) -> Vec<Position> {
    let mut position = start;
    let mut path = vec![*start];
    while let Some(next) = path_map.get(position) {
        path.push(*next);
        position = next;
    }
    path.reverse();
    path
}

fn find_path(
    start: (usize, usize),
    end: (usize, usize),
    map_grid: &FxHashSet<(usize, usize)>,
) -> Option<Vec<Position>> {
    let node = Node(start, 0);
    let mut frontier = BinaryHeap::new();
    frontier.push(node);
    let mut expanded = HashSet::new();

    let bounds = end.0;
    let mut path_map = FxHashMap::default();

    while let Some(node) = frontier.pop() {
        let Node(position, _) = node.clone();
        if position == end {
            // won
            return Some(traverse_path(&path_map, &position));
        }
        let neighbours = neighbourhood(map_grid, &node, bounds as isize);
        let mut hash_frontier: FxHashMap<_, _> =
            frontier.into_iter().map(|f| ((f.0), (f.1))).collect();

        for neighbour in neighbours {
            if !expanded.contains(&(neighbour.0)) {
                let entry = hash_frontier.entry(neighbour.0);
                match entry {
                    Entry::Occupied(mut entry) => {
                        let val = entry.get_mut();
                        if *val > neighbour.1 {
                            *val = neighbour.1;
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(neighbour.1);
                    }
                }
                path_map.insert(neighbour.0, node.0);
            }
        }
        // print_map_path(map_grid, bounds, &node.1, &hash_frontier);
        frontier = hash_frontier.into_iter().map(|(k, v)| Node(k, v)).collect();
        expanded.insert(node.0);
    }
    None
}

#[allow(unused)]
fn print_map_path(
    map_grid: &FxHashSet<Position>,
    bounds: usize,
    node_path: &[Position],
    frontier: &FxHashMap<Position, (Vec<(usize, usize)>, u64)>,
) {
    let mut s = String::new();
    for y in 0..=bounds {
        for x in 0..=bounds {
            let letter = match map_grid.contains(&(x, y)) {
                true => '#',
                false => {
                    if node_path.contains(&(x, y)) {
                        '$'
                    } else if frontier.contains_key(&(x, y)) {
                        '?'
                    } else {
                        '.'
                    }
                }
            };
            s.push(letter);
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn neighbourhood(map_grid: &FxHashSet<Position>, node: &Node, bounds: isize) -> Vec<Node> {
    let (node_x, node_y) = node.0;
    let mut new = Vec::new();

    for (rel_x, rel_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let x = (node_x as isize) + rel_x;
        let y = (node_y as isize) + rel_y;
        if x < 0 || y < 0 || x > bounds || y > bounds {
            continue;
        }
        let position = (x as usize, y as usize);
        if map_grid.contains(&position) {
            continue;
        }
        new.push(Node(position, node.1 + 1));
    }

    new
}

type Position = (usize, usize);

fn parse(contents: String) -> Vec<Position> {
    let mut positions = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        positions.push((x, y));
    }
    positions
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/18/real.txt").unwrap()),
            "318"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/18/real.txt").unwrap()),
            "56,29"
        );
    }
}

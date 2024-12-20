use std::collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet};

use rustc_hash::FxHashMap;

pub fn part1(contents: String) -> String {
    let (map_grid, start, end) = parse(contents);

    let node = find_path(start, end, &map_grid);

    let path = node.unwrap().1;

    let path_cost: HashMap<Position, usize> = path.iter().cloned().collect();

    let jumps: Vec<(isize, isize)> = vec![(0, -2), (0, 2), (-2, 0), (2, 0)];

    let mut good_cheat = 0;

    for (from, from_cost) in path {
        for jump in jumps.clone() {
            let to = (
                (from.0 as isize + jump.0) as usize,
                (from.1 as isize + jump.1) as usize,
            );
            if let Some(&to_cost) = path_cost.get(&to) {
                // jump costs 2
                let diff = from_cost as isize - to_cost as isize - 2;
                if diff >= 100 {
                    good_cheat += 1;
                }
            }
        }
    }

    good_cheat.to_string()
}

pub fn part2(contents: String) -> String {
    let (map_grid, start, end) = parse(contents);

    let node = find_path(start, end, &map_grid);

    let path = node.unwrap().1;

    let path_cost: HashMap<Position, usize> = path.iter().cloned().collect();

    let mut good_cheat = 0;

    let mut relative_positions = HashSet::new();
    for jump in 1..=20 {
        // can jump to any euclidean distance up to 20
        for t in 0..=jump {
            let u = jump - t;
            if t > u {
                break;
            }
            // down right
            relative_positions.insert(((t, u), jump));
            relative_positions.insert(((u, t), jump));

            // down left
            relative_positions.insert(((-t, u), jump));
            relative_positions.insert(((-u, t), jump));

            // up right
            relative_positions.insert(((t, -u), jump));
            relative_positions.insert(((u, -t), jump));

            // up left
            relative_positions.insert(((-t, -u), jump));
            relative_positions.insert(((-u, -t), jump));
        }
    }

    for (from, from_cost) in path {
        for (jump, jump_cost) in relative_positions.clone() {
            let to = (
                (from.0 as isize + jump.0) as usize,
                (from.1 as isize + jump.1) as usize,
            );
            if let Some(&to_cost) = path_cost.get(&to) {
                let diff = from_cost as isize - to_cost as isize - jump_cost;
                if diff >= 100 {
                    good_cheat += 1;
                }
            }
        }
    }

    good_cheat.to_string()
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Node(Position, Vec<(Position, usize)>, usize);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.2.cmp(&self.2))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}

fn find_path(
    start: (usize, usize),
    end: (usize, usize),
    map_grid: &Vec<Vec<bool>>,
) -> Option<Node> {
    let node = Node(start, vec![(start, 0)], 0);
    let mut frontier = BinaryHeap::new();
    frontier.push(node);
    let mut expanded = HashSet::new();

    while let Some(node) = frontier.pop() {
        let Node(position, _, _) = node.clone();
        if position == end {
            // won
            // print_map_path(map_grid, &node.1);
            return Some(node);
        }
        let neighbours = neighbourhood(&map_grid, &node);
        let mut hash_frontier: FxHashMap<_, _> =
            frontier.into_iter().map(|f| ((f.0), (f.1, f.2))).collect();
        for neighbour in neighbours {
            if !expanded.contains(&(neighbour.0)) {
                let entry = hash_frontier.entry(neighbour.0);
                match entry {
                    Entry::Occupied(mut entry) => {
                        let val = entry.get_mut();
                        if val.1 > neighbour.2 {
                            *val = (neighbour.1, neighbour.2);
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert((neighbour.1, neighbour.2));
                    }
                }
            }
        }

        frontier = hash_frontier
            .into_iter()
            .map(|(k, v)| Node(k, v.0, v.1))
            .collect();
        expanded.insert(node.0);
    }
    None
}

#[allow(unused)]
fn print_map_path(map_grid: &[Vec<bool>], node_path: &Vec<(Position, usize)>) {
    let positions: Vec<_> = node_path.iter().cloned().map(|(p, _)| p).collect();
    println!("\n\n");
    let mut s = String::new();
    for y in 0..map_grid.len() {
        for x in 0..map_grid[0].len() {
            let letter = match map_grid[y][x] {
                true => '#',
                false => {
                    if positions.contains(&(x, y)) {
                        '$'
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

fn neighbourhood(map_grid: &Vec<Vec<bool>>, node: &Node) -> Vec<Node> {
    let (node_x, node_y) = node.0;
    let mut new = Vec::new();

    for (rel_x, rel_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let x = (node_x as isize) + rel_x;
        let y = (node_y as isize) + rel_y;
        if x < 0 || y < 0 || x > map_grid[0].len() as isize || y > map_grid.len() as isize {
            continue;
        }
        let position = (x as usize, y as usize);
        if map_grid[y as usize][x as usize] {
            continue;
        }
        let mut new_path = node.1.clone();
        let cost = node.2 + 1;
        new_path.push((position, cost));
        new.push(Node(position, new_path, cost));
    }

    new
}

type Position = (usize, usize);

fn parse(contents: String) -> (Vec<Vec<bool>>, Position, Position) {
    let mut map_grid = Vec::new();
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        let map_line: Vec<_> = line
            .chars()
            .enumerate()
            .map(|(x, c)| match c {
                '#' => true,
                '.' => false,
                'S' => {
                    start_position = (x, y);
                    false
                }
                'E' => {
                    end_position = (x, y);
                    false
                }
                _ => panic!("Unknown block"),
            })
            .collect();
        map_grid.push(map_line);
    }
    (map_grid, start_position, end_position)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/20/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/20/real.txt").unwrap()),
            "example2"
        );
    }
}

use std::{
    collections::{hash_map::Entry, BinaryHeap, HashSet},
    hash::Hash,
};

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Node(Position, Direction, Vec<(Position, u64)>, u64);

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.3.partial_cmp(&self.3)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.3.cmp(&self.3)
    }
}

pub fn part1(contents: String) -> String {
    let (map_grid, start, end) = parse(contents);
    let direction = Direction::Right;

    let node = find_path(start, end, direction, &map_grid);

    node.3.to_string()
}

pub fn part2(contents: String) -> String {
    let (map_grid, start, end) = parse(contents);
    let direction = Direction::Right;

    let node = find_path(start, end, direction, &map_grid);

    let goals = node.2.into_iter().collect();
    let all = find_all_paths(start, goals, direction, map_grid);

    all.to_string()
}

fn find_path(
    start: (usize, usize),
    end: (usize, usize),
    direction: Direction,
    map_grid: &Vec<Vec<bool>>,
) -> Node {
    let node = Node(start, direction, vec![(start, 0)], 0);
    let mut frontier = BinaryHeap::new();
    frontier.push(node);
    let mut expanded = HashSet::new();

    while let Some(node) = frontier.pop() {
        let Node(position, _, _path, _) = node.clone();
        if position == end {
            // won
            return node;
        }
        let neighbours = neighbourhood(&map_grid, &node);
        let mut hash_frontier: FxHashMap<_, _> = frontier
            .into_iter()
            .map(|f| ((f.0, f.1), (f.2, f.3)))
            .collect();
        for neighbour in neighbours {
            if !expanded.contains(&(neighbour.0, neighbour.1)) {
                let entry = hash_frontier.entry((neighbour.0, neighbour.1));
                match entry {
                    Entry::Occupied(mut entry) => {
                        let val = entry.get_mut();
                        if val.1 > neighbour.3 {
                            *val = (neighbour.2, neighbour.3);
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert((neighbour.2, neighbour.3));
                    }
                }
            }
        }
        frontier = hash_frontier
            .into_iter()
            .map(|(k, v)| Node(k.0, k.1, v.0, v.1))
            .collect();
        expanded.insert((node.0, node.1));
        // print_map(&map_grid, position, direction);
    }
    panic!("Unable to find path")
}

fn find_all_paths(
    start: (usize, usize),
    goals: FxHashSet<(Position, u64)>,
    direction: Direction,
    map_grid: Vec<Vec<bool>>,
) -> usize {
    let node = Node(start, direction, vec![(start, 0)], 0);
    let mut frontier = BinaryHeap::new();
    frontier.push(node);
    let mut expanded = FxHashMap::default();

    let mut best_path = FxHashSet::default();

    while let Some(node) = frontier.pop() {
        if goals.contains(&(node.0, node.3)) {
            for (p, _) in &node.2 {
                best_path.insert(*p);
            }
        }
        if let Some(cost) = expanded.get(&(node.0, node.1)) {
            if *cost < node.3 {
                continue;
            }
        }
        let neighbours = neighbourhood(&map_grid, &node);
        for neighbour in neighbours {
            frontier.push(neighbour);
        }
        expanded.insert((node.0, node.1), node.3);
    }

    best_path.len()
}

#[allow(unused)]
fn print_map_path(
    map_grid: &[Vec<bool>],
    node_path: &HashSet<&(usize, usize)>,
    frontier: &HashSet<(usize, usize)>,
    best_path: &HashSet<(usize, usize), rustc_hash::FxBuildHasher>,
    direction: Direction,
) {
    println!("Direction: {:#?}", direction);
    let mut s = String::new();
    for y in 0..map_grid.len() {
        for x in 0..map_grid[0].len() {
            let letter = match map_grid[y][x] {
                true => '#',
                false => {
                    if node_path.contains(&(x, y)) {
                        '$'
                    } else if best_path.contains(&(x, y)) {
                        'O'
                    } else if frontier.contains(&(x, y)) {
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

fn neighbourhood(map_grid: &[Vec<bool>], node: &Node) -> Vec<Node> {
    let mut new = Vec::new();
    let new_directions = match node.1 {
        Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
        Direction::Right | Direction::Left => (Direction::Up, Direction::Down),
    };
    for new_direction in [new_directions.0, new_directions.1] {
        new.push(Node(node.0, new_direction, node.2.clone(), node.3 + 1000));
    }
    let new_position = next_position(node);
    if !map_grid[new_position.1][new_position.0] {
        let mut path = node.2.clone();
        path.push((new_position, node.3 + 1));
        new.push(Node(new_position, node.1, path, node.3 + 1));
    }

    new
}

fn next_position(node: &Node) -> (usize, usize) {
    let position = node.0;
    let (new_x, new_y) = match node.1 {
        Direction::Up => (position.0, position.1 - 1),
        Direction::Right => (position.0 + 1, position.1),
        Direction::Down => (position.0, position.1 + 1),
        Direction::Left => (position.0 - 1, position.1),
    };
    (new_x, new_y)
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
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

#[allow(dead_code)]
fn print_map(map_grid: &[Vec<bool>], current_position: Position, direction: Direction) {
    println!("Direction: {:#?}", direction);
    let mut s = String::new();
    for y in 0..map_grid.len() {
        for x in 0..map_grid[0].len() {
            let letter = match map_grid[y][x] {
                true => '#',
                false => {
                    if (x, y) == current_position {
                        '@'
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/16/real.txt").unwrap()),
            "101492"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/16/real.txt").unwrap()),
            "543"
        );
    }
}

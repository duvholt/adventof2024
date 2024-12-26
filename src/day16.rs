use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Node(
    (Position, Direction),
    Option<((Position, Direction), u64)>,
    u64,
);

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

pub fn part1(contents: String) -> String {
    let (map_grid, start, end) = parse(contents);
    let direction = Direction::Right;

    let (node, _) = find_path(start, end, direction, &map_grid, false);

    node.2.to_string()
}

pub fn part2(contents: String) -> String {
    let (map_grid, start, end) = parse(contents);
    let direction = Direction::Right;

    let (_, path) = find_path(start, end, direction, &map_grid, true);

    let goals = path.unwrap().into_iter().collect();
    let all = find_all_paths(start, goals, direction, map_grid);

    all.to_string()
}

fn find_path(
    start: (usize, usize),
    end: (usize, usize),
    direction: Direction,
    map_grid: &Vec<Vec<bool>>,
    include_path: bool,
) -> (Node, Option<Vec<(Position, u64)>>) {
    let node = Node((start, direction), None, 0);
    let mut frontier = BinaryHeap::new();
    frontier.push(node);
    let mut expanded = HashSet::new();
    let mut path_map = FxHashMap::default();

    while let Some(node) = frontier.pop() {
        if expanded.contains(&node.0) {
            continue;
        }

        if include_path {
            if let Some(prev) = node.1 {
                path_map.insert(node.0, prev);
            }
        }
        if node.0 .0 == end {
            // won
            let path = if include_path {
                Some(traverse_path(&path_map, &node))
            } else {
                None
            };
            return (node, path);
        }
        let neighbours = neighbourhood(map_grid, &node);
        for neighbour in neighbours {
            if !expanded.contains(&neighbour.0) {
                frontier.push(neighbour);
            }
        }
        expanded.insert(node.0);
        // print_map(&map_grid, position, direction);
    }
    panic!("Unable to find path")
}

fn traverse_path(
    path_map: &FxHashMap<(Position, Direction), ((Position, Direction), u64)>,
    start: &Node,
) -> Vec<(Position, u64)> {
    let mut position = &(start.0, start.2);
    let mut path = vec![(position.0 .0, start.2)];
    while let Some(next) = path_map.get(&position.0) {
        path.push((next.0 .0, next.1));
        position = next;
    }
    path.reverse();
    path
}

fn find_all_paths(
    start: (usize, usize),
    goals: FxHashSet<(Position, u64)>,
    direction: Direction,
    map_grid: Vec<Vec<bool>>,
) -> usize {
    let node = Node((start, direction), None, 0);
    let mut frontier = BinaryHeap::new();
    frontier.push(node);
    let mut expanded = FxHashMap::default();

    let mut path_map = FxHashMap::default();
    let mut best_path = FxHashSet::default();
    best_path.extend(goals.iter().cloned().map(|g| g.0));

    while let Some(node) = frontier.pop() {
        let visited_node = expanded.get(&node.0);
        let already_visited = visited_node.is_some();
        if let Some(cost) = visited_node {
            if *cost < node.2 {
                continue;
            }
        }
        if let Some(prev) = node.1 {
            path_map.insert(node.0, prev);
        }

        if already_visited {
            if goals.contains(&(node.0 .0, node.2)) {
                best_path.extend(traverse_path(&path_map, &node).into_iter().map(|p| p.0));
            }
            continue;
        }
        let neighbours = neighbourhood(&map_grid, &node);
        for neighbour in neighbours {
            if !expanded.contains_key(&neighbour.0) {
                frontier.push(neighbour);
            }
        }
        expanded.insert(node.0, node.2);
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
    let new_directions = match node.0 .1 {
        Direction::Up | Direction::Down => (Direction::Left, Direction::Right),
        Direction::Right | Direction::Left => (Direction::Up, Direction::Down),
    };
    let new_position = next_position(node);
    if !map_grid[new_position.1][new_position.0] {
        new.push(Node(
            (new_position, node.0 .1),
            Some((node.0, node.2)),
            node.2 + 1,
        ));
    }
    for new_direction in [new_directions.0, new_directions.1] {
        new.push(Node(
            (node.0 .0, new_direction),
            Some((node.0, node.2)),
            node.2 + 1000,
        ));
    }

    new
}

fn next_position(node: &Node) -> (usize, usize) {
    let position = node.0 .0;
    let (new_x, new_y) = match node.0 .1 {
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

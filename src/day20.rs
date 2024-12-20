use std::collections::VecDeque;

use rustc_hash::{FxHashMap, FxHashSet};

pub fn part1(contents: String) -> String {
    let (map_grid, start, end) = parse(contents);

    let node = find_path(start, end, &map_grid);

    let path = node.unwrap();
    let path_cost: FxHashMap<Position, usize> = path.iter().cloned().collect();

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

    let path = find_path(start, end, &map_grid);

    let path = path.unwrap();

    let path_cost: FxHashMap<Position, usize> = path.iter().cloned().collect();

    let mut good_cheat = 0;

    let mut relative_positions = FxHashSet::default();
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

#[derive(Debug)]
struct Node(Position, usize);

fn find_path(
    start: (usize, usize),
    end: (usize, usize),
    map_grid: &[Vec<bool>],
) -> Option<Vec<(Position, usize)>> {
    let estimated_size = map_grid.len().pow(2) / 2;
    let node = Node(start, 0);
    let mut frontier = VecDeque::with_capacity(estimated_size);
    frontier.push_front(node);
    let mut expanded =
        FxHashSet::with_capacity_and_hasher(estimated_size, rustc_hash::FxBuildHasher);

    let mut path_map = FxHashMap::default();

    while let Some(node) = frontier.pop_back() {
        if node.0 == end {
            // print_map_path(map_grid, &node.1);
            return Some(traverse_path(&path_map, &(node.0, node.1)));
        }
        let neighbours = neighbourhood(map_grid, &node);
        for neighbour in neighbours {
            if !expanded.contains(&(neighbour.0)) {
                path_map.insert(neighbour.0, (node.0, node.1));
                frontier.push_front(neighbour);
            }
        }
        expanded.insert(node.0);
    }
    None
}

fn traverse_path(
    path_map: &FxHashMap<Position, (Position, usize)>,
    start: &(Position, usize),
) -> Vec<(Position, usize)> {
    let mut position = start.0;
    let mut path = vec![*start];
    while let Some(next) = path_map.get(&position) {
        path.push(*next);
        position = next.0;
    }
    path.reverse();
    path
}

#[allow(unused)]
fn print_map_path(map_grid: &[Vec<bool>], node_path: &[(Position, usize)]) {
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

fn neighbourhood(map_grid: &[Vec<bool>], node: &Node) -> Vec<Node> {
    let (node_x, node_y) = node.0;
    let mut new = Vec::with_capacity(4);

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
        let cost = node.1 + 1;
        new.push(Node(position, cost));
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
            "1293"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/20/real.txt").unwrap()),
            "977747"
        );
    }
}

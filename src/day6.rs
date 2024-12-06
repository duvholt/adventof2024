use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Grid = Vec<Vec<char>>;

fn direction_to_rel(dir: &Direction) -> (isize, isize) {
    match dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

pub fn part1(contents: String) -> String {
    let grid: Grid = contents.lines().map(|l| l.chars().collect()).collect();
    let max_x: isize = grid[0].len() as isize;
    let max_y: isize = grid.len() as isize;
    let visited = find_visited_positions(max_y, max_x, &grid);

    // print_map(&grid, &visited);
    visited.len().to_string()
}

fn find_visited_positions(max_y: isize, max_x: isize, grid: &Grid) -> HashSet<(isize, isize)> {
    let mut visited = HashSet::new();
    let mut direction = Direction::Up;

    let mut position = find_start_position(max_y, max_x, grid);

    loop {
        visited.insert(position);
        let rel = direction_to_rel(&direction);
        let new_pos = ((position.0 + rel.0) as isize, (position.1 + rel.1) as isize);
        if bounds_check(new_pos, max_x, max_y) {
            break;
        }
        let infront = grid[new_pos.1 as usize][new_pos.0 as usize];
        if infront == '#' {
            direction = rotate(&direction)
        } else {
            position = new_pos;
        }
    }
    visited
}

pub fn part2(contents: String) -> String {
    let grid: Grid = contents.lines().map(|l| l.chars().collect()).collect();
    let max_x: isize = grid[0].len() as isize;
    let max_y: isize = grid.len() as isize;
    let start_position = find_start_position(max_y, max_x, &grid);

    let mut loops = 0;

    let visited = find_visited_positions(max_y, max_x, &grid);

    // lets go bruteforcing
    for (x, y) in visited {
        // skip start
        if start_position.0 == x && start_position.1 == y {
            continue;
        }
        let looped = find_loop(x, y, max_x, max_y, start_position, &grid);
        if looped {
            loops += 1;
        }
    }

    loops.to_string()
}

fn bounds_check(new_pos: (isize, isize), max_x: isize, max_y: isize) -> bool {
    new_pos.0 < 0 || new_pos.0 >= max_x || new_pos.1 < 0 || new_pos.1 >= max_y
}

fn rotate(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn find_start_position(max_y: isize, max_x: isize, grid: &Grid) -> (isize, isize) {
    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y as usize][x as usize] == '^' {
                return (x, y);
            }
        }
    }
    panic!("Unable to find guard");
}

fn find_loop(
    x: isize,
    y: isize,
    max_x: isize,
    max_y: isize,
    start_position: (isize, isize),
    grid: &Grid,
) -> bool {
    let new_block = (x, y);

    let mut position = start_position;
    let mut direction = Direction::Up;
    let mut looped = false;
    let mut visited_with_dir =
        HashSet::with_capacity_and_hasher((x * y) as usize, rustc_hash::FxBuildHasher);

    loop {
        let rel = direction_to_rel(&direction);
        let new_pos = ((position.0 + rel.0) as isize, (position.1 + rel.1) as isize);
        if bounds_check(new_pos, max_x, max_y) {
            break;
        }
        let infront = grid[new_pos.1 as usize][new_pos.0 as usize];
        if infront == '#' || new_pos == new_block {
            let pos_with_dir = (position, direction.clone());
            if visited_with_dir.contains(&pos_with_dir) {
                // loopy
                looped = true;
                break;
            }
            visited_with_dir.insert(pos_with_dir);

            direction = rotate(&direction)
        } else {
            position = new_pos;
        }
    }
    looped
}

#[allow(dead_code)]
fn print_map(grid: &Grid, visited: &HashSet<(isize, isize)>) {
    for y in 0..grid.len() {
        let mut line = String::new();
        for x in 0..grid[0].len() {
            if visited.contains(&(x as isize, y as isize)) {
                line.push('X');
            } else {
                line.push(grid[y][x]);
            }
        }
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/6/real.txt").unwrap()),
            "4977"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/6/real.txt").unwrap()),
            "1729"
        );
    }
}

use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn direction_to_rel(dir: &Direction) -> (isize, isize) {
    match dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    }
}

pub fn part1(contents: String) -> String {
    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let max_x: isize = grid[0].len() as isize;
    let max_y: isize = grid.len() as isize;
    let mut visited = HashSet::new();
    let mut position: (isize, isize) = (0, 0);
    let mut direction = Direction::Up;

    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y as usize][x as usize] == '^' {
                position = (x, y);
                break;
            }
        }
    }

    loop {
        visited.insert(position);
        let rel = direction_to_rel(&direction);
        let new_pos = ((position.0 + rel.0) as isize, (position.1 + rel.1) as isize);
        if new_pos.0 < 0 || new_pos.0 >= max_x || new_pos.1 < 0 || new_pos.1 >= max_y {
            break;
        }
        let infront = grid[new_pos.1 as usize][new_pos.0 as usize];
        if infront == '#' {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        } else {
            position = new_pos;
        }
    }

    print_map(grid, &visited);
    visited.len().to_string()
}

pub fn part2(contents: String) -> String {
    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let max_x: isize = grid[0].len() as isize;
    let max_y: isize = grid.len() as isize;
    let mut start_position: (isize, isize) = (0, 0);

    for y in 0..max_y {
        for x in 0..max_x {
            if grid[y as usize][x as usize] == '^' {
                start_position = (x, y);
                break;
            }
        }
    }

    let mut loops = 0;

    for y in 0..max_y {
        for x in 0..max_x {
            // skip start
            if start_position.0 == x && start_position.1 == y {
                continue;
            }
            let mut position = start_position;
            let mut direction = Direction::Up;
            let mut looped = false;
            let mut visited_with_dir = HashSet::new();

            loop {
                let pos_with_dir = (position, direction.clone());
                if visited_with_dir.contains(&pos_with_dir) {
                    // loopy
                    looped = true;
                    break;
                }
                visited_with_dir.insert(pos_with_dir);
                let rel = direction_to_rel(&direction);
                let new_pos = ((position.0 + rel.0) as isize, (position.1 + rel.1) as isize);
                if new_pos.0 < 0 || new_pos.0 >= max_x || new_pos.1 < 0 || new_pos.1 >= max_y {
                    break;
                }
                let infront = grid[new_pos.1 as usize][new_pos.0 as usize];
                if infront == '#' || new_pos == (x, y) {
                    direction = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                    }
                } else {
                    position = new_pos;
                }
            }
            if looped {
                loops += 1;
            }
        }
    }

    // print_map(grid, &visited_with_dir);
    loops.to_string()
}

fn print_map(grid: Vec<Vec<char>>, visited: &HashSet<(isize, isize)>) {
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
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/6/real.txt").unwrap()),
            "example2"
        );
    }
}

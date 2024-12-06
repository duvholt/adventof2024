use std::collections::HashSet;

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

pub fn part2(_contents: String) -> String {
    "example2".to_string()
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

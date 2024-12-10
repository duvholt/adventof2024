use std::collections::HashSet;

pub fn part1(contents: String) -> String {
    let grid: Vec<Vec<u64>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();

    let start_points = find_start_points(&grid);

    let mut sum = 0;

    for start_point in start_points {
        let mut peaks = 0;

        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push(start_point);
        while let Some(point) = stack.pop() {
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point);
            if grid[point.1][point.0] == 9 {
                peaks += 1;
            }
            for n in neighbourhood(&grid, point) {
                stack.push(n);
            }
        }

        sum += peaks;
    }

    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let grid: Vec<Vec<u64>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();

    let start_points = find_start_points(&grid);

    let mut sum = 0;

    for start_point in start_points {
        let mut peaks = 0;

        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        stack.push((start_point, Vec::new()));
        while let Some(point_with_path) = stack.pop() {
            if visited.contains(&point_with_path) {
                continue;
            }
            visited.insert(point_with_path.clone());
            let (point, path) = point_with_path.clone();
            if grid[point.1][point.0] == 9 {
                peaks += 1;
                continue;
            }
            let mut new_path = path;
            new_path.push(point);
            for n in neighbourhood(&grid, point) {
                stack.push((n, new_path.clone()));
            }
        }

        sum += peaks;
    }

    sum.to_string()
}

fn find_start_points(grid: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut start_points = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                start_points.push((x, y));
            }
        }
    }
    start_points
}

fn neighbourhood(grid: &[Vec<u64>], point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut hood = Vec::new();
    let (x, y) = point;
    let value = grid[y][x];
    if x > 0 {
        let new = (x - 1, y);
        add_if_can_climb(grid, new, value, &mut hood);
    }
    if y > 0 {
        let new = (x, y - 1);
        add_if_can_climb(grid, new, value, &mut hood);
    }
    if x < grid[0].len() - 1 {
        let new = (x + 1, y);
        add_if_can_climb(grid, new, value, &mut hood);
    }
    if y < grid.len() - 1 {
        let new = (x, y + 1);
        add_if_can_climb(grid, new, value, &mut hood);
    }
    hood
}

fn add_if_can_climb(
    grid: &[Vec<u64>],
    new: (usize, usize),
    value: u64,
    hood: &mut Vec<(usize, usize)>,
) {
    if grid[new.1][new.0] == value + 1 {
        hood.push(new);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/10/real.txt").unwrap()),
            "644"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/10/real.txt").unwrap()),
            "1366"
        );
    }
}

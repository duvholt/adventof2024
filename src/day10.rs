use std::collections::HashSet;

pub fn part1(contents: String) -> String {
    let grid: Vec<Vec<u64>> = contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();

    let mut start_points = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                start_points.push((x, y));
            }
        }
    }

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
            visited.insert(point.clone());
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

pub fn part2(_contents: String) -> String {
    "example2".to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/10/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/10/real.txt").unwrap()),
            "example2"
        );
    }
}

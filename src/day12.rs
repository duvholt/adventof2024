use std::collections::HashSet;

pub fn part1(contents: String) -> String {
    let grid = parse(contents);

    let areas = find_areas(grid);
    areas
        .into_iter()
        .map(|(area, perimeter)| perimeter as u64 * area.len() as u64)
        .sum::<u64>()
        .to_string()
}

pub fn part2(contents: String) -> String {
    let grid = parse(contents);

    let areas = find_areas(grid);
    let mut sum = 0;
    for (points, _) in areas {
        let points: HashSet<_> = points.into_iter().collect();
        let (min_x, max_x, min_y, max_y) = find_bounds(&points);
        // up edges
        let mut up_edges = 0;
        let mut down_edges = 0;
        for y in min_y..=max_y {
            let mut has_prev_up = false;
            let mut has_prev_down = false;
            for x in min_x..=max_x {
                let point = (x, y);
                if !points.contains(&point) {
                    has_prev_up = false;
                    has_prev_down = false;
                    continue;
                }
                // up edges
                if (y == 0) || !points.contains(&(x, y - 1)) {
                    if !has_prev_up {
                        up_edges += 1;
                        has_prev_up = true;
                    }
                } else {
                    has_prev_up = false;
                }
                // down
                if !points.contains(&(x, y + 1)) {
                    if !has_prev_down {
                        down_edges += 1;
                        has_prev_down = true;
                    }
                } else {
                    has_prev_down = false;
                }
            }
        }

        let mut left_edges = 0;
        let mut right_edges = 0;
        for x in min_x..=max_x {
            let mut has_prev_left = false;
            let mut has_prev_right = false;
            for y in min_y..=max_y {
                let point = (x, y);
                if !points.contains(&point) {
                    has_prev_left = false;
                    has_prev_right = false;
                    continue;
                }
                // left edges
                if (x == 0) || !points.contains(&(x - 1, y)) {
                    if !has_prev_left {
                        left_edges += 1;
                        has_prev_left = true;
                    }
                } else {
                    has_prev_left = false;
                }
                // right
                if !points.contains(&(x + 1, y)) {
                    if !has_prev_right {
                        right_edges += 1;
                        has_prev_right = true;
                    }
                } else {
                    has_prev_right = false;
                }
            }
        }

        sum += points.len() * (up_edges + down_edges + right_edges + left_edges);
    }
    sum.to_string()
}

fn find_bounds(points: &HashSet<(usize, usize)>) -> (usize, usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;
    for &(x, y) in points.iter() {
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }
    (min_x, max_x, min_y, max_y)
}

fn parse(contents: String) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    grid
}

fn find_areas(grid: Vec<Vec<char>>) -> Vec<(Vec<(usize, usize)>, usize)> {
    let mut visited = HashSet::new();
    let mut areas = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let point = (x, y);
            if visited.contains(&point) {
                continue;
            }
            let mut perimeter = 0;
            let mut area = Vec::new();
            let mut stack = Vec::new();
            stack.push(point);
            while let Some(stack_point) = stack.pop() {
                if visited.contains(&stack_point) {
                    continue;
                }
                visited.insert(stack_point);
                let neighbours = neighbourhood(&grid, stack_point);
                perimeter += 4 - neighbours.len();
                area.push(stack_point);
                for n in neighbours {
                    if !visited.contains(&n) {
                        stack.push(n);
                    }
                }
            }
            areas.push((area, perimeter));
        }
    }
    areas
}

fn neighbourhood(grid: &[Vec<char>], point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut hood = Vec::new();
    let (x, y) = point;
    let value = grid[y][x];
    if x > 0 {
        let new = (x - 1, y);
        add_if_same(grid, new, value, &mut hood);
    }
    if y > 0 {
        let new = (x, y - 1);
        add_if_same(grid, new, value, &mut hood);
    }
    if x < grid[0].len() - 1 {
        let new = (x + 1, y);
        add_if_same(grid, new, value, &mut hood);
    }
    if y < grid.len() - 1 {
        let new = (x, y + 1);
        add_if_same(grid, new, value, &mut hood);
    }
    hood
}

fn add_if_same(
    grid: &[Vec<char>],
    new: (usize, usize),
    value: char,
    hood: &mut Vec<(usize, usize)>,
) {
    if grid[new.1][new.0] == value {
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
            part1(fs::read_to_string("./input/12/real.txt").unwrap()),
            "1494342"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/12/real.txt").unwrap()),
            "893676"
        );
    }
}

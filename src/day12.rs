use std::collections::HashSet;

pub fn part1(contents: String) -> String {
    let grid = parse(contents);

    let areas = find_areas(grid);
    areas
        .into_iter()
        .map(|(perimeter, area)| perimeter * area as u64)
        .sum::<u64>()
        .to_string()
}

pub fn part2(contents: String) -> String {
    todo!()
}

fn parse(contents: String) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    grid
}

fn find_areas(grid: Vec<Vec<char>>) -> Vec<(u64, usize)> {
    let mut visited = HashSet::new();
    let mut areas = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let point = (x, y);
            if visited.contains(&point) {
                continue;
            }
            let mut perimeter = 0;
            let mut area = 0;
            let mut stack = Vec::new();
            stack.push(point);
            while let Some(stack_point) = stack.pop() {
                if visited.contains(&stack_point) {
                    continue;
                }
                visited.insert(stack_point);
                let neighbours = neighbourhood(&grid, stack_point);
                perimeter += 4 - neighbours.len();
                area += 1;
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
            "example2"
        );
    }
}

use std::collections::{HashMap, HashSet};

pub fn part1(contents: String) -> String {
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let max_x = grid[0].len() as isize;
    let max_y = grid.len() as isize;
    let grid_map: HashMap<(isize, isize), char> = grid
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter().enumerate().filter_map(move |(x, char)| {
                if char == '.' {
                    None
                } else {
                    Some(((x as isize, y as isize), char))
                }
            })
        })
        .collect();
    let mut reverse_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (&pos, &freq) in grid_map.iter() {
        let entry = reverse_map.entry(freq).or_default();
        entry.push(pos);
    }

    let mut antinodes = HashSet::new();

    for (freq, values) in reverse_map {
        for value1 in values.iter() {
            for value2 in values.iter() {
                if value1 == value2 {
                    continue;
                }

                let (x1, y1) = *value1;
                let (x2, y2) = *value2;
                let diff = (x2 - x1, y2 - y1);

                let antinode1 = (x1 + diff.0 * 2, y1 + diff.1 * 2);
                if inbounds(antinode1, max_x, max_y) {
                    antinodes.insert(antinode1);
                }
                let antinode2 = (x2 - diff.0 * 2, y2 - diff.1 * 2);
                if inbounds(antinode2, max_x, max_y) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }

    antinodes.len().to_string()
}

pub fn part2(contents: String) -> String {
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let max_x = grid[0].len() as isize;
    let max_y = grid.len() as isize;
    let grid_map: HashMap<(isize, isize), char> = grid
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter().enumerate().filter_map(move |(x, char)| {
                if char == '.' {
                    None
                } else {
                    Some(((x as isize, y as isize), char))
                }
            })
        })
        .collect();
    let mut reverse_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (&pos, &freq) in grid_map.iter() {
        let entry = reverse_map.entry(freq).or_default();
        entry.push(pos);
    }

    let mut antinodes = HashSet::new();

    for (freq, values) in reverse_map {
        for value1 in values.iter() {
            for value2 in values.iter() {
                if value1 == value2 {
                    continue;
                }

                let (x1, y1) = *value1;
                let (x2, y2) = *value2;
                let diff = (x2 - x1, y2 - y1);

                let mut antinode1 = (x2, y2);
                while inbounds(antinode1, max_x, max_y) {
                    antinodes.insert(antinode1);
                    antinode1 = (antinode1.0 + diff.0, antinode1.1 + diff.1);
                }

                let mut antinode2 = (x1, y1);
                while inbounds(antinode2, max_x, max_y) {
                    antinodes.insert(antinode2);
                    antinode2 = (antinode2.0 - diff.0, antinode2.1 - diff.1);
                }
            }
        }
    }

    antinodes.len().to_string()
}

fn inbounds(pos: (isize, isize), max_x: isize, max_y: isize) -> bool {
    pos.0 >= 0 && pos.0 < max_x && pos.1 >= 0 && pos.1 < max_y
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/8/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/8/real.txt").unwrap()),
            "example2"
        );
    }
}

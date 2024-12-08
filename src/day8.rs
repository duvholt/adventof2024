use std::collections::{HashMap, HashSet};

pub fn part1(contents: String) -> String {
    let (max_x, max_y, freq_map) = parse(contents);

    let mut antinodes = HashSet::new();

    for (_freq, values) in freq_map {
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
    let (max_x, max_y, freq_map) = parse(contents);

    let mut antinodes = HashSet::new();

    for (_freq, values) in freq_map {
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

type FrequencyMap = HashMap<char, Vec<(isize, isize)>>;

fn parse(contents: String) -> (isize, isize, FrequencyMap) {
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let max_x = grid[0].len() as isize;
    let max_y = grid.len() as isize;

    let mut freq_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (y, line) in grid.into_iter().enumerate() {
        for (x, freq) in line.into_iter().enumerate() {
            if freq != '.' {
                let pos = (x as isize, y as isize);
                let entry = freq_map.entry(freq).or_default();
                entry.push(pos);
            }
        }
    }
    (max_x, max_y, freq_map)
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
            "351"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/8/real.txt").unwrap()),
            "1259"
        );
    }
}

pub fn part1(contents: String) -> String {
    let (height, keys, holes) = parse(contents);

    let fit: usize = keys
        .iter()
        .map(|key| {
            holes
                .iter()
                .filter(|hole| (0..hole.len()).all(|x| key[x] + hole[x] < height))
                .count()
        })
        .sum();

    fit.to_string()
}

fn parse(contents: String) -> (i32, Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut height = 0;
    let key_holes = contents.split("\n\n").map(|l| {
        let lines: Vec<Vec<char>> = l.lines().map(|s| s.chars().collect()).collect();
        let key_or_hole = lines[0] == ['#', '#', '#', '#', '#'];
        // subtract filled row
        height = lines.len() as i32 - 1;
        let mut pin_size = vec![-1; lines[0].len()];
        for x in 0..lines[0].len() {
            for line in lines.iter() {
                if line[x] == '#' {
                    pin_size[x] += 1;
                }
            }
        }
        (key_or_hole, pin_size)
    });

    let (keys, holes): (Vec<_>, Vec<_>) = key_holes.partition(|(key_or_hole, _)| *key_or_hole);
    let keys: Vec<_> = keys.into_iter().map(|(_, key)| key).collect();
    let holes: Vec<_> = holes.into_iter().map(|(_, hole)| hole).collect();
    (height, keys, holes)
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
            part1(fs::read_to_string("./input/25/real.txt").unwrap()),
            "2618"
        );
    }
}

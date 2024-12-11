// If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.

pub fn part1(_contents: String) -> String {
    let mut stones: Vec<u64> = _contents
        .split_ascii_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    for iteration in 0..75 {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }
            let stone_s = stone.to_string();
            if stone_s.len() % 2 == 0 {
                let middle = stone_s.len() / 2;
                let l = &stone_s[0..middle];
                let r = &stone_s[middle..];
                stones.insert(i, l.parse().unwrap());
                i += 1;
                stones[i] = r.parse().unwrap();
                i += 1;
                continue;
            } else {
                stones[i] *= 2024;
                i += 1;
            }
        }
        dbg!(iteration, stones.len());
    }
    stones.len().to_string()
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
            part1(fs::read_to_string("./input/11/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/11/real.txt").unwrap()),
            "example2"
        );
    }
}

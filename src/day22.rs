// Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
// Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
// Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.

// To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
// To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)

use rustc_hash::{FxHashMap, FxHashSet};

pub fn part1(contents: String) -> String {
    let initial_secrets: Vec<u64> = contents.lines().map(|l| l.parse().unwrap()).collect();

    let mut sum = 0;
    for initial in initial_secrets {
        let mut secret = initial;
        for _i in 0..2000 {
            let mult = secret * 64;
            secret ^= mult;
            secret %= 16777216;

            let div = secret / 32;
            secret ^= div;
            secret %= 16777216;

            let mult2 = secret * 2048;
            secret ^= mult2;
            secret %= 16777216;
        }
        sum += secret;
    }
    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let initial_secrets: Vec<u64> = contents.lines().map(|l| l.parse().unwrap()).collect();

    let mut secret_changes = Vec::new();

    for initial in initial_secrets {
        let mut secret = initial;
        let mut prev_digit: i8 = (secret % 10).try_into().unwrap();

        let mut changes = Vec::new();

        for _i in 0..2000 {
            let mult = secret * 64;
            secret ^= mult;
            secret %= 16777216;

            let div = secret / 32;
            secret ^= div;
            secret %= 16777216;

            let mult2 = secret * 2048;
            secret ^= mult2;
            secret %= 16777216;

            let last_digit = (secret % 10).try_into().unwrap();
            let change = last_digit - prev_digit;
            changes.push((last_digit as u8, change));
            prev_digit = last_digit;
        }
        secret_changes.push(changes);
    }

    let mut seq_scores: FxHashMap<[i8; 4], u64> = FxHashMap::default();

    for changes in secret_changes.iter() {
        let mut unique = FxHashSet::default();
        for w in changes.windows(4) {
            let seq = [w[0].1, w[1].1, w[2].1, w[3].1];
            if unique.contains(&seq) {
                continue;
            }
            unique.insert(seq);
            let score = w[3].0 as u64;
            *seq_scores.entry(seq).or_default() += score;
        }
    }
    seq_scores.values().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/22/real.txt").unwrap()),
            "16894083306"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/22/real.txt").unwrap()),
            "1925"
        );
    }
}

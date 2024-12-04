pub fn part1(contents: String) -> String {
    let word = "XMAS";
    let rev_word = "SAMX";
    let mut sum = 0;
    let lines: Vec<_> = contents.lines().collect();
    let max_x = lines[0].len();
    let max_y = lines.len();
    println!("{} {}", max_x, max_y);
    // horizontal
    for line in lines.iter() {
        println!("Line: {}", line);
        let v = find_xmas(line, word);
        sum += v.len();
        let v_rev: Vec<_> = find_xmas(line, rev_word);
        sum += v_rev.len();
        println!("Line matches {:?} / {:?}", v, v_rev);
    }
    // vertical
    for x in 0..max_x {
        let column: String = (0..max_x)
            .map(|y1| lines[y1].as_bytes()[x] as char)
            .collect();
        let (v, v_rev) = xmas_sum(&column, word, rev_word, &mut sum);
        println!("Line matches {:?} => {:?} / {:?}", column, v, v_rev);
    }

    // diagonal down left / up right
    for start_x in 0..max_x {
        let column: String = (0..max_x - start_x)
            .map(|x1| {
                let x = start_x + x1;
                let y = x1;
                lines[y].as_bytes()[x] as char
            })
            .collect();
        let (v, v_rev) = xmas_sum(&column, word, rev_word, &mut sum);
        println!("Line matches {:?} => {:?} / {:?}", column, v, v_rev);
    }

    // diagonal down right / up
    for start_x in 0..max_x {
        let column: String = (0..max_x - start_x)
            .map(|x1| {
                let x = (max_x - 1) - x1 - start_x;
                let y = x1;
                lines[y].as_bytes()[x] as char
            })
            .collect();
        let (v, v_rev) = xmas_sum(&column, word, rev_word, &mut sum);
        // println!("Line matches {:?} => {:?} / {:?}", column, v, v_rev);
    }

    // down left, down left
    // up left, up right
    sum.to_string()
}

fn xmas_sum<'a>(
    column: &'a str,
    word: &str,
    rev_word: &str,
    sum: &mut usize,
) -> (Vec<(usize, &'a str)>, Vec<(usize, &'a str)>) {
    let v = find_xmas(column, word);
    let v_rev: Vec<_> = find_xmas(column, rev_word);
    *sum += v.len();
    *sum += v_rev.len();
    (v, v_rev)
}

fn find_xmas<'a>(line: &'a str, word: &str) -> Vec<(usize, &'a str)> {
    line.match_indices(word).collect()
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
            part1(fs::read_to_string("./input/4/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/4/real.txt").unwrap()),
            "example2"
        );
    }
}

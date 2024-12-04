pub fn part1(contents: String) -> String {
    let word = "XMAS";
    let rev_word = "SAMX";
    let mut sum = 0;
    let lines: Vec<_> = contents.lines().collect();
    let max_x = lines[0].len();
    let max_y = lines.len();
    // println!("{} {}", max_x, max_y);
    // horizontal
    for line in lines.iter() {
        // println!("Line: {}", line);
        let v = find_xmas(line, word);
        sum += v.len();
        let v_rev: Vec<_> = find_xmas(line, rev_word);
        sum += v_rev.len();
        // println!("Line matches {:?} / {:?}", v, v_rev);
    }
    // vertical
    for x in 0..max_x {
        let column: String = (0..max_x)
            .map(|y1| lines[y1].as_bytes()[x] as char)
            .collect();
        let (v, v_rev) = xmas_sum(&column, word, rev_word, &mut sum);
        // println!("Line matches {:?} => {:?} / {:?}", column, v, v_rev);
    }

    // diagonal down left / up right
    // todo: missing: y
    for start_x in 0..max_x {
        let end = max_x - start_x;
        let column: String = (0..end)
            .map(|x1| {
                let x = start_x + x1;
                let y = x1;
                lines[y].as_bytes()[x] as char
            })
            .collect();

        xmas_sum(&column, word, rev_word, &mut sum);
        // println!("Diagonal x 1 {:?}", column);
    }
    for start_y in 0..max_y {
        let end = max_y - start_y;
        let column: String = (1..end)
            .map(|x1| {
                let x = x1 - 1;
                let y = start_y + x1;
                lines[y].as_bytes()[x] as char
            })
            .collect();
        xmas_sum(&column, word, rev_word, &mut sum);
        // println!("Diagonal y 1 {:?}", column);
    }

    // diagonal down right / up
    for start_x in 0..max_x {
        let end = max_x - start_x;
        let column: String = (0..end)
            .map(|x1| {
                let x = (max_x - 1) - start_x - x1;
                let y = x1;
                lines[y].as_bytes()[x] as char
            })
            .collect();

        xmas_sum(&column, word, rev_word, &mut sum);
        // println!("Line matches {:?} => {:?} / {:?}", column, v, v_rev);
        // println!("Diagonal x 2 {:?}", column);
    }
    for start_y in 0..max_y {
        let end = max_y - start_y;
        let column: String = (1..end)
            .map(|x1| {
                let x = (max_x - 1) - x1 + 1;
                let y = start_y + x1;
                lines[y].as_bytes()[x] as char
            })
            .collect();
        xmas_sum(&column, word, rev_word, &mut sum);
        // println!("Diagonal y 2 {:?}", column);
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

pub fn part2(contents: String) -> String {
    let lines: Vec<Vec<_>> = contents
        .lines()
        .map(|l| l.as_bytes().iter().map(|c| *c as char).collect())
        .collect();
    let max_x = lines[0].len();
    let max_y = lines.len();
    let mut sum = 0;
    for x in 0..max_x - 2 {
        for y in 0..max_y - 2 {
            let center = lines[y + 1][x + 1];
            if center != 'A' {
                continue;
            }
            let corner_ul = lines[y][x];
            let corner_dr = lines[y + 2][x + 2];
            let mut first = [corner_ul, corner_dr].to_vec();
            first.sort();
            let corner_ur = lines[y][x + 2];
            let corner_dl = lines[y + 2][x];
            let mut second = [corner_ur, corner_dl].to_vec();
            second.sort();
            if first == ['M', 'S'] && second == ['M', 'S'] {
                sum += 1;
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/4/real.txt").unwrap()),
            "2530"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/4/real.txt").unwrap()),
            "1921"
        );
    }
}

#[derive(Debug, Clone)]
enum Node {
    Block(usize, u32),
    Empty(u32),
}

pub fn part1(contents: String) -> String {
    let nodes: Vec<Node> = contents
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let length = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                Node::Block(i / 2, length)
            } else {
                Node::Empty(length)
            }
        })
        .collect();
    let mut disk_map = Vec::new();
    let mut disk_length = 0;
    for node in nodes {
        match node {
            Node::Block(id, length) => {
                disk_length += length;
                for _ in 0..length {
                    disk_map.push(Some(id));
                }
            }
            Node::Empty(length) => {
                for _ in 0..length {
                    disk_map.push(None);
                }
            }
        }
    }
    let mut compacted_disk_map = vec![0; disk_length as usize];
    // let mut disk_i: usize = 0;
    let mut disk_j: usize = disk_map.len() - 1;
    for i in 0..disk_length as usize {
        let disk_value = disk_map[i];
        match disk_value {
            Some(id) => {
                compacted_disk_map[i] = id;
                // disk_i += 1;
            }
            None => {
                let mut end = None;
                while end.is_none() {
                    end = disk_map[disk_j];
                    disk_j -= 1;
                }
                compacted_disk_map[i] = end.unwrap();
            }
        }
    }

    let sum: usize = compacted_disk_map
        .into_iter()
        .enumerate()
        .map(|(i, value)| i * value)
        .sum();
    sum.to_string()
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
            part1(fs::read_to_string("./input/9/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/9/real.txt").unwrap()),
            "example2"
        );
    }
}

#[derive(Debug, Clone)]
enum Node {
    Block(usize, u32),
    Empty(u32),
}

pub fn part1(contents: String) -> String {
    let nodes = parse(contents);
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
    let mut disk_j: usize = disk_map.len() - 1;
    for i in 0..disk_length as usize {
        let disk_value = disk_map[i];
        match disk_value {
            Some(id) => {
                compacted_disk_map[i] = id;
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

pub fn part2(contents: String) -> String {
    let mut nodes = parse(contents);

    let mut compacted_nodes = vec![];

    // let mut disk_i: usize = 0;
    for i in 0..nodes.len() {
        if i >= nodes.len() {
            break;
        }
        let node = nodes[i].clone();
        match node {
            Node::Empty(empty_length) => {
                fill(&mut nodes, empty_length, &mut compacted_nodes, i);
            }
            Node::Block(id, length) => {
                compacted_nodes.push((id, length));
            }
        }
    }

    let mut i: u64 = 0;
    let mut sum: u64 = 0;
    for (id, length) in compacted_nodes {
        if id > 1000000 {
            i += length as u64;
        } else {
            for _ in 0..length {
                sum += i * id as u64;
                i += 1;
            }
        }
    }

    sum.to_string()
}

fn fill(
    nodes: &mut Vec<Node>,
    empty_length: u32,
    compacted_nodes: &mut Vec<(usize, u32)>,
    i: usize,
) {
    let mut end = None;
    let mut j = nodes.len() - 1;
    while end.is_none() {
        end = match nodes[j].clone() {
            Node::Block(id, length) => {
                if length <= empty_length {
                    Some((id, length))
                } else {
                    None
                }
            }
            Node::Empty(_) => None,
        };
        if end.is_some() {
            nodes.remove(j);
            nodes.insert(j, Node::Empty(end.unwrap().1));
        }
        if j == 0 || j <= i {
            break;
        }
        j -= 1;
    }
    if let Some(end) = end {
        compacted_nodes.push(end);
        let remaining = empty_length - end.1;
        if remaining > 0 {
            fill(nodes, remaining, compacted_nodes, i);
        }
    } else {
        compacted_nodes.push((usize::MAX, empty_length));
    }
}

fn parse(contents: String) -> Vec<Node> {
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
    nodes
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/9/real.txt").unwrap()),
            "6283404590840"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/9/real.txt").unwrap()),
            "6304576012713"
        );
    }
}

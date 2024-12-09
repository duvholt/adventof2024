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
    let mut compacted_disk_map = Vec::with_capacity(disk_length as usize);
    let mut disk_j: usize = disk_map.len() - 1;
    for i in 0..disk_length as usize {
        let disk_value = disk_map[i];
        match disk_value {
            Some(id) => {
                compacted_disk_map.push(id);
            }
            None => {
                let mut end = None;
                while end.is_none() {
                    end = disk_map[disk_j];
                    disk_j -= 1;
                }
                compacted_disk_map.push(end.unwrap());
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

    let mut compacted_nodes = Vec::with_capacity(nodes.len() / 2);

    for i in 0..nodes.len() {
        if i >= nodes.len() {
            break;
        }
        let node = nodes[i].clone();
        match node {
            Node::Empty(empty_length) => {
                defragment(&mut compacted_nodes, &mut nodes, empty_length, i);
            }
            Node::Block(id, length) => {
                compacted_nodes.push((Some(id), length));
            }
        }
    }

    let mut i: u32 = 0;
    let mut sum: u64 = 0;
    for (node, length) in compacted_nodes {
        match node {
            Some(id) => {
                for _ in 0..length {
                    sum += i as u64 * id as u64;
                    i += 1;
                }
            }
            None => i += length,
        }
    }

    sum.to_string()
}

fn defragment(
    compacted_nodes: &mut Vec<(Option<usize>, u32)>,
    nodes: &mut [Node],
    empty_length: u32,
    i: usize,
) {
    let mut remaining = empty_length;
    while remaining > 0 {
        let block = find_block(nodes, remaining, i);
        if let Some(((id, length), j)) = block {
            nodes[j] = Node::Empty(length);
            compacted_nodes.push((Some(id), length));
            remaining -= length;
        } else {
            break;
        }
    }
    if remaining > 0 {
        compacted_nodes.push((None, remaining));
    }
}

fn find_block(nodes: &[Node], empty_length: u32, i: usize) -> Option<((usize, u32), usize)> {
    let mut j = nodes.len() - 1;
    loop {
        match nodes[j] {
            Node::Block(id, length) => {
                if length <= empty_length {
                    return Some(((id, length), j));
                }
            }
            Node::Empty(_) => {}
        };
        if j == 0 || j <= i {
            break;
        }
        j -= 1;
    }
    None
}

fn parse(contents: String) -> Vec<Node> {
    let nodes: Vec<Node> = contents
        .chars()
        .filter(|c| !c.is_whitespace())
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

use std::collections::{HashMap, HashSet};

pub fn part1(contents: String) -> String {
    let edges: Vec<_> = contents
        .lines()
        .map(|l| {
            let mut p = l.split("-");
            (p.next().unwrap(), p.next().unwrap())
        })
        .collect();

    let mut nodes: HashSet<String> = HashSet::new();
    let mut directed_edges: HashMap<String, HashSet<String>> = HashMap::default();
    for (from, to) in edges.iter() {
        nodes.insert(from.to_string());
        nodes.insert(to.to_string());

        let from_set = directed_edges.entry(from.to_string()).or_default();
        from_set.insert(to.to_string());

        let to_set = directed_edges.entry(to.to_string()).or_default();
        to_set.insert(from.to_string());
    }

    let mut cliques = Vec::new();

    let mut visited = HashSet::new();

    for (i1, node1) in nodes.iter().enumerate() {
        let node1_set = directed_edges.get(node1).unwrap();
        for (i2, node2) in nodes.iter().enumerate() {
            if i2 <= i1 {
                continue;
            }
            if !node1_set.contains(node2) {
                continue;
            }
            if visited.contains(&(node1, node2)) || visited.contains(&(node2, node1)) {
                continue;
            }

            let node2_set = directed_edges.get(node2).unwrap();

            let intersection: Vec<_> = node1_set.intersection(node2_set).collect();

            visited.insert((node1, node2));
            visited.insert((node2, node1));
            for node3 in intersection {
                if visited.contains(&(node1, node3)) || visited.contains(&(node2, node3)) {
                    continue;
                }

                visited.insert((node1, node2));
                visited.insert((node2, node1));
                cliques.push((node1, node2, node3));
            }
        }
    }

    let mut sum = 0;
    for clique in cliques {
        if clique.0.starts_with("t") || clique.1.starts_with("t") || clique.2.starts_with("t") {
            sum += 1;
        }
    }

    sum.to_string()
}

pub fn part2(contents: String) -> String {
    let edges: Vec<_> = contents
        .lines()
        .map(|l| {
            let mut p = l.split("-");
            (p.next().unwrap(), p.next().unwrap())
        })
        .collect();

    let mut nodes: HashSet<String> = HashSet::new();
    let mut directed_edges: HashMap<String, HashSet<String>> = HashMap::default();
    for (from, to) in edges.iter() {
        nodes.insert(from.to_string());
        nodes.insert(to.to_string());

        let from_set = directed_edges.entry(from.to_string()).or_default();
        from_set.insert(to.to_string());

        let to_set = directed_edges.entry(to.to_string()).or_default();
        to_set.insert(from.to_string());
    }

    let mut largest_clique = HashSet::default();

    for node in nodes.iter() {
        let mut remaining_nodes: Vec<_> = nodes.clone().into_iter().collect();
        let mut clique = HashSet::new();
        clique.insert(node.to_string());
        while let Some(vertice) = remaining_nodes.pop() {
            let edges = directed_edges.get(&vertice).unwrap();
            if edges.is_superset(&clique) {
                clique.insert(vertice.to_string());
            }
        }

        if clique.len() > largest_clique.len() {
            largest_clique = clique;
        }
    }

    let mut clique_nodes: Vec<_> = largest_clique.into_iter().collect();
    clique_nodes.sort();

    clique_nodes.join(",")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/23/real.txt").unwrap()),
            "example"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/23/real.txt").unwrap()),
            "example2"
        );
    }
}

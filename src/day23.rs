use rustc_hash::{FxHashMap, FxHashSet};

pub fn part1(contents: String) -> String {
    let edges = parse(&contents);

    let (nodes, directed_edges) = find_nodes_and_edges(edges);

    let mut three_cliques = 0;

    let mut visited = FxHashSet::default();

    for node1 in nodes.iter() {
        let node1_set = directed_edges.get(node1).unwrap();
        let starts_with_t1 = node1.starts_with("t");
        for node2 in node1_set.iter() {
            let starts_with_t2 = node2.starts_with("t");
            visited.insert((node1, node2));
            visited.insert((node2, node1));

            let node2_set = directed_edges.get(node2).unwrap();
            let node3_set = node1_set.intersection(node2_set);

            for node3 in node3_set {
                if !(starts_with_t1 || starts_with_t2 || node3.starts_with("t")) {
                    continue;
                }
                if visited.contains(&(node1, node3)) || visited.contains(&(node2, node3)) {
                    continue;
                }
                three_cliques += 1;
            }
        }
    }

    three_cliques.to_string()
}

fn find_nodes_and_edges<'a>(
    edges: Vec<(&'a str, &'a str)>,
) -> (FxHashSet<&'a str>, FxHashMap<&'a str, FxHashSet<&'a str>>) {
    let mut nodes: FxHashSet<&str> = FxHashSet::default();
    let mut directed_edges: FxHashMap<&str, FxHashSet<&str>> = FxHashMap::default();
    for (from, to) in edges.iter() {
        nodes.insert(from);
        nodes.insert(to);

        let from_set = directed_edges.entry(from).or_default();
        from_set.insert(to);

        let to_set = directed_edges.entry(to).or_default();
        to_set.insert(from);
    }
    (nodes, directed_edges)
}

pub fn part2(contents: String) -> String {
    let edges = parse(&contents);

    let (nodes, directed_edges) = find_nodes_and_edges(edges);

    let mut largest_clique = FxHashSet::default();

    for node in nodes.iter().cloned() {
        let mut clique = FxHashSet::default();
        clique.insert(node);
        for vertice in nodes.iter() {
            let edges = directed_edges.get(vertice).unwrap();
            if edges.is_superset(&clique) {
                clique.insert(vertice);
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

fn parse(contents: &str) -> Vec<(&str, &str)> {
    let edges: Vec<_> = contents
        .lines()
        .map(|l| {
            let mut p = l.split("-");
            (p.next().unwrap(), p.next().unwrap())
        })
        .collect();
    edges
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(fs::read_to_string("./input/23/real.txt").unwrap()),
            "1512"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(fs::read_to_string("./input/23/real.txt").unwrap()),
            "ac,ed,fh,kd,lf,mb,om,pe,qt,uo,uy,vr,wg"
        );
    }
}

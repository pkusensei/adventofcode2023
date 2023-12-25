#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

fn solve(input: &str) -> usize {
    let mut graph = parse(input);
    for _ in 0..3 {
        let (left, right) = find_link(&graph);
        graph.get_mut(&left).map(|v| v.remove(&right));
        graph.get_mut(&right).map(|v| v.remove(&left));
    }
    let size = bfs(&graph);
    size * (graph.len() - size)
}

fn bfs(graph: &HashMap<&str, HashSet<&str>>) -> usize {
    let start = graph.keys().next().cloned().unwrap();
    let mut queue = VecDeque::from([start]);
    let mut seen = HashSet::new();
    while let Some(curr) = queue.pop_front() {
        if seen.insert(curr) {
            queue.extend(graph[&curr].iter().cloned())
        }
    }
    seen.len()
}

fn find_link<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> (&'a str, &'a str) {
    let mut links: HashMap<(&str, &str), usize> = HashMap::new();
    for start in graph.keys().cloned() {
        let mut queue = VecDeque::from([start]);
        let mut seen = HashSet::from([start]);
        while let Some(curr) = queue.pop_front() {
            for &node in graph[&curr].iter() {
                if !seen.insert(node) {
                    continue;
                }
                queue.push_back(node);
                let link = if curr < node {
                    (curr, node)
                } else {
                    (node, curr)
                };
                *links.entry(link).or_default() += 1;
            }
        }
    }
    links
        .into_iter()
        .max_by_key(|(_k, v)| *v)
        .map(|(k, _v)| k)
        .unwrap()
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph: HashMap<&str, HashSet<_>> = HashMap::new();
    for line in input.lines() {
        let mut it = line.split(": ");
        let left = it.next().unwrap().trim();
        for right in it.next().unwrap().split_whitespace() {
            graph.entry(right).or_default().insert(left);
            graph.entry(left).or_default().insert(right);
        }
    }
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST), 54);
        assert_eq!(solve(INPUT), 543834);
    }
}

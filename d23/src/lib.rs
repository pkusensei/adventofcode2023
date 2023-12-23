#![allow(dead_code)]

use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    convert::identity,
};

use utils::{Coord, Dir};

fn p1(input: &str) -> usize {
    let (start, goal, grid) = parse(input);
    longest(start, goal, HashSet::new(), &grid).len() - 1
}

fn p2(input: &str) -> usize {
    let (start, goal, grid) = parse(input);

    let graph = condense(start, goal, grid);

    let mut longest = 0;
    let mut queue = VecDeque::from([(start, 0, HashSet::new())]);
    while let Some((curr, dist, mut seen)) = queue.pop_front() {
        if curr == goal {
            longest = cmp::max(longest, dist);
            continue;
        }

        if seen.contains(&curr) {
            continue;
        }
        seen.insert(curr);
        queue.extend(
            graph[&curr]
                .iter()
                .map(|&(next, next_dist)| (next, next_dist + dist, seen.clone())),
        );
    }

    longest
}

fn condense(
    start: Coord,
    goal: Coord,
    grid: HashMap<Coord, u8>,
) -> HashMap<Coord, Vec<(Coord, usize)>> {
    // Nodes with >3 neighbors; one is previous step
    let mut nodes: Vec<_> = grid
        .iter()
        .filter_map(|(&k, &v)| if v != b'#' { Some(k) } else { None })
        .filter(|&node| proceed2(node, &HashSet::new(), &grid).len() > 2)
        .collect();
    nodes.extend([start, goal]);

    let mut seen = HashSet::new();
    // bi-directional graph
    // node: Vec<(neighbor, distance)>
    let mut graph = HashMap::<Coord, Vec<(Coord, usize)>>::new();
    for &node in &nodes {
        let mut queue = VecDeque::from([(node, 0)]);
        while let Some((curr, dist)) = queue.pop_front() {
            if nodes.contains(&curr) && curr != node {
                // Walked to a node that branches
                // Skips all paths from node curr
                graph.entry(node).or_default().push((curr, dist));
                graph.entry(curr).or_default().push((node, dist));
                continue;
            }
            if seen.contains(&curr) {
                continue;
            }
            seen.insert(curr);
            queue.extend(
                proceed2(curr, &seen, &grid)
                    .into_iter()
                    .map(|c| (c, dist + 1)),
            );
        }
    }
    graph
}

fn longest(
    start: Coord,
    goal: Coord,
    mut seen: HashSet<Coord>,
    grid: &HashMap<Coord, u8>,
) -> HashSet<Coord> {
    let mut queue = VecDeque::from([start]);
    while let Some(curr) = queue.pop_front() {
        if seen.contains(&curr) {
            continue;
        }
        seen.insert(curr);
        if curr == goal {
            return seen;
        }
        let neighbors = proceed(curr, &seen, grid);
        if neighbors.len() > 1 {
            if let Some(s) = neighbors
                .into_iter()
                .filter_map(|c| {
                    let s = longest(c, goal, seen.clone(), grid);
                    if s.contains(&goal) {
                        Some(s)
                    } else {
                        None
                    }
                })
                .max_by_key(|s| s.len())
            {
                seen = s;
            }
        } else {
            queue.extend(neighbors)
        }
    }
    seen
}

fn proceed(curr: Coord, seen: &HashSet<Coord>, grid: &HashMap<Coord, u8>) -> Vec<Coord> {
    utils::deltas(curr.0, curr.1)
        .into_iter()
        .filter_map(move |(c, dir)| match (grid[&curr], dir) {
            (b'^', Dir::North) | (b'v', Dir::South) | (b'<', Dir::West) | (b'>', Dir::East)
                if !seen.contains(&c) && grid.get(&c).is_some_and(|&v| v != b'#') =>
            {
                Some(c)
            }
            (b'.', _) if !seen.contains(&c) && grid.get(&c).is_some_and(|&v| v != b'#') => Some(c),
            _ => None,
        })
        .collect()
}

fn proceed2(curr: Coord, seen: &HashSet<Coord>, grid: &HashMap<Coord, u8>) -> Vec<Coord> {
    utils::deltas(curr.0, curr.1)
        .into_iter()
        .filter_map(|(c, _)| {
            if c != curr && !seen.contains(&c) && grid.get(&c).is_some_and(|&v| v != b'#') {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}

fn parse(input: &str) -> (Coord, Coord, HashMap<Coord, u8>) {
    let (lens, it) = utils::parse_with_lens(input, &identity);
    let start = input
        .lines()
        .next()
        .and_then(|line| line.trim().as_bytes().iter().position(|&b| b == b'.'))
        .unwrap();
    let end = input
        .lines()
        .last()
        .and_then(|line| line.trim().as_bytes().iter().position(|&b| b == b'.'))
        .unwrap();
    ((start, 0), (end, lens.1 - 1), it.collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"#.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 94);
        assert_eq!(p1(INPUT), 2154);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 154);
        assert_eq!(p2(INPUT), 6654);
    }
}

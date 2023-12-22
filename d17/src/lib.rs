#![allow(dead_code)]

use std::collections::{BinaryHeap, HashMap};

use utils::{Coord, Dir};

fn p1(input: &str) -> u32 {
    let ((x, y), grid) = parse(input);
    dijkstra((0, 0), (x - 1, y - 1), (x, y), 1, 3, grid).unwrap()
}

fn p2(input: &str) -> u32 {
    let ((x, y), grid) = parse(input);
    dijkstra((0, 0), (x - 1, y - 1), (x, y), 4, 10, grid).unwrap()
}

// Rust doc on BinaryHeap
// https://doc.rust-lang.org/std/collections/binary_heap/#examples
fn dijkstra(
    start: Coord,
    end: Coord,
    lens: Coord,
    min_step: u8,
    max_step: u8,
    grid: HashMap<Coord, u32>,
) -> Option<u32> {
    let mut dist = HashMap::<StateKey, u32>::new();
    let mut heap = BinaryHeap::new();
    let st1 = State {
        cost: 0,
        pos: start,
        dir: Dir::East,
        steps: 0,
    };
    let st2 = State {
        dir: Dir::South,
        ..st1
    };
    dist.insert(st1.into(), 0);
    dist.insert(st2.into(), 0);
    heap.push(st1);
    heap.push(st2);

    while let Some(
        state @ State {
            cost,
            pos,
            dir,
            steps,
        },
    ) = heap.pop()
    {
        if pos == end && steps >= min_step {
            return Some(cost);
        }
        if dist.get(&state.into()).is_some_and(|&c| c < cost) {
            continue;
        }
        for (n, d) in neighbors(pos, dir, lens) {
            let next = State {
                cost: cost + grid[&n],
                pos: n,
                dir: d,
                steps: if d == dir { steps + 1 } else { 1 },
            };
            if next.steps > max_step || dist.get(&next.into()).is_some_and(|&c| c <= next.cost) {
                // For p1 and p2
                // Streak gets too long
                // Or a better solution/path already exists
                continue;
            }
            if next.dir != dir && steps < min_step {
                // For p2
                // Making a turn but previous streak is too short
                continue;
            }
            heap.push(next);
            dist.insert(next.into(), next.cost);
        }
    }
    None
}

fn neighbors((x, y): Coord, dir: Dir, (x_len, y_len): Coord) -> Vec<(Coord, Dir)> {
    utils::deltas(x, y)
        .into_iter()
        .filter_map(|(n, d)| {
            if n == (x, y) || n.0 > x_len - 1 || n.1 > y_len - 1 || d == dir.flip() {
                None
            } else {
                Some((n, d))
            }
        })
        .collect()
}

fn parse(input: &str) -> (Coord, HashMap<Coord, u32>) {
    let (lens, it) = utils::parse_with_lens(input, &|b| (b - b'0').into());
    (lens, it.collect())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    pos: Coord,
    dir: Dir,
    steps: u8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct StateKey {
    pos: Coord,
    dir: Dir,
    steps: u8,
}

impl From<State> for StateKey {
    fn from(value: State) -> Self {
        Self {
            pos: value.pos,
            dir: value.dir,
            steps: value.steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533"#;

    const TEST2: &'static str = r#"111111111111
    999999999991
    999999999991
    999999999991
    999999999991"#;

    const TEST3: &'static str = r#"19999
    19999
    19999
    19999
    11111"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 102);
        assert_eq!(p1(INPUT), 1244);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 94);
        assert_eq!(p2(TEST2), 71);
        assert_eq!(p2(TEST3), 8);
        assert_eq!(p2(INPUT), 1367);
    }
}

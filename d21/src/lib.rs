#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    convert::identity,
};

use itertools::Itertools;

use utils::Coord;

fn p1(input: &str, steps: usize) -> usize {
    let (_, start, grid) = parse(input);
    let mut curr = vec![start];
    let mut i = 0;
    while i < steps {
        curr = curr
            .into_iter()
            .flat_map(|c| proceed(c, &grid))
            .unique()
            .collect();
        i += 1;
    }
    curr.len()
}

// Still grappling with this one.
// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
fn p2(input: &str) -> (usize, usize) {
    let (lens, start, grid) = parse(input);
    let mut seen = HashMap::new();
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((curr, dist)) = queue.pop_front() {
        if seen.contains_key(&curr) {
            continue;
        }
        seen.insert(curr, dist);
        queue.extend(proceed(curr, &grid).filter_map(|(x, y)| {
            if x < lens.0 && y < lens.1 && !seen.contains_key(&(x, y)) {
                Some(((x, y), dist + 1))
            } else {
                None
            }
        }));
    }

    const STEPS: usize = 26501365;
    debug_assert_eq!(lens.0, lens.1);
    debug_assert_eq!(lens.0, 131);

    // Number of grids covered to one direction
    let count = (STEPS - lens.0 / 2) / lens.0;
    debug_assert_eq!(count, 202300);

    // Tiles within 64 steps and step number is even
    let p1 = seen
        .values()
        .filter(|&&v| v < lens.0 / 2 && v & 1 == 0)
        .count();

    // Tiles >65 steps away and step number is even
    let even_corners = seen
        .values()
        .filter(|&&v| v > lens.0 / 2 && v & 1 == 0)
        .count();
    let odd_corners = seen
        .values()
        .filter(|&&v| v > lens.0 / 2 && v & 1 == 1)
        .count();

    let even = count * count;
    let odd = (count + 1) * (count + 1);
    let p2 = even * seen.values().filter(|&&v| v & 1 == 0).count()
        + odd * seen.values().filter(|&&v| v & 1 == 1).count()
        + count * even_corners
        - (count + 1) * odd_corners;

    (p1, p2)
}

fn proceed(curr: Coord, grid: &HashMap<Coord, u8>) -> impl Iterator<Item = Coord> + '_ {
    utils::deltas(curr.0, curr.1)
        .into_iter()
        .map(|(c, _)| c)
        .filter(move |&c| c != curr && grid.get(&c).is_some_and(|&v| v != b'#'))
}

fn parse(input: &str) -> (Coord, Coord, HashMap<Coord, u8>) {
    let (lens, it) = utils::parse_with_lens(input, &identity);
    let grid: HashMap<_, _> = it.collect();
    let start = grid
        .iter()
        .find_map(|(&k, &v)| if v == b'S' { Some(k) } else { None })
        .unwrap_or_default();
    (lens, start, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ..........."#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST, 6), 16);
        assert_eq!(p1(INPUT, 64), 3748);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(INPUT), (3748, 616951804315987));
    }
}

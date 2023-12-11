#![allow(dead_code)]

use std::{cmp, collections::BTreeSet};

use itertools::Itertools;

fn p1(input: &str) -> usize {
    solve(input, 2)
}

fn p2(input: &str, expand: usize) -> usize {
    solve(input, expand)
}

fn solve(input: &str, expand: usize) -> usize {
    let ((x_max, y_max), galaxies) = parse(input);
    let empty_rows = (0..=y_max)
        .filter(|&y| (0..=x_max).all(|x| !galaxies.contains(&(x, y))))
        .collect::<BTreeSet<usize>>();
    let empty_cols = (0..=x_max)
        .filter(|&x| (0..=y_max).all(|y| !galaxies.contains(&(x, y))))
        .collect::<BTreeSet<usize>>();
    galaxies
        .into_iter()
        .combinations(2)
        .map(|v| {
            manhattan_dist(&v[0], &v[1])
                + expand_count(v[0].0, v[1].0, &empty_cols) * (expand - 1)
                + expand_count(v[0].1, v[1].1, &empty_rows) * (expand - 1)
        })
        .sum()
}

type Coord = (usize, usize);

fn manhattan_dist(c1: &Coord, c2: &Coord) -> usize {
    c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1)
}

fn expand_count(a: usize, b: usize, empty: &BTreeSet<usize>) -> usize {
    (cmp::min(a, b)..cmp::max(a, b))
        .collect::<BTreeSet<_>>()
        .intersection(empty)
        .count()
}

fn parse(input: &str) -> (Coord, BTreeSet<Coord>) {
    let y_max = input.lines().count();
    let x_max = input.lines().map(|line| line.len()).max().unwrap();
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.bytes().enumerate().map(move |(x, b)| ((x, y), b)))
        .filter_map(|(c, b)| if b == b'#' { Some(c) } else { None })
        .collect();
    ((x_max, y_max), galaxies)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 374);
        assert_eq!(p1(INPUT), 10490062);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST, 10), 1030);
        assert_eq!(p2(TEST, 100), 8410);
        assert_eq!(p2(INPUT, 1000000), 382979724122);
    }
}

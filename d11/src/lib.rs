#![allow(dead_code)]

use std::{cmp, collections::HashSet, convert::identity};

use itertools::Itertools;

use utils::Coord;

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
        .collect::<Vec<_>>();
    let empty_cols = (0..=x_max)
        .filter(|&x| (0..=y_max).all(|y| !galaxies.contains(&(x, y))))
        .collect::<Vec<_>>();
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

const fn manhattan_dist(c1: &Coord, c2: &Coord) -> usize {
    c1.0.abs_diff(c2.0) + c1.1.abs_diff(c2.1)
}

fn expand_count(a: usize, b: usize, empty: &[usize]) -> usize {
    empty
        .iter()
        .filter(|&&n| cmp::min(a, b) < n && n < cmp::max(a, b))
        .count()
}

fn parse(input: &str) -> (Coord, HashSet<Coord>) {
    let (lens, it) = utils::parse_with_lens(input, &identity);
    let galaxies = it
        .filter_map(|(c, b)| if b == b'#' { Some(c) } else { None })
        .collect();
    (lens, galaxies)
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

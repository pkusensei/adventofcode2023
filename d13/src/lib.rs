#![allow(dead_code)]

use std::{cmp, collections::HashMap, convert::identity};

use itertools::Itertools;
use regex::Regex;

use utils::Coord;

fn p1(input: &str) -> usize {
    solve(input, find)
}

fn p2(input: &str) -> usize {
    solve(input, find2)
}

fn solve(input: &str, find: FindFn) -> usize {
    let re = Regex::new(r#"(?m)^\s*$\n?"#).unwrap();
    parse(input, &re)
        .filter_map(|(c, map)| {
            let (x, y) = find(c, &map);
            x.map(|v| v + 1).or_else(|| y.map(|v| 100 * (v + 1)))
        })
        .sum()
}

fn find((x_len, y_len): Coord, map: &HashMap<Coord, u8>) -> (Option<usize>, Option<usize>) {
    let x = (0..x_len - 1).find(|x| {
        let range = cmp::min(x + 1, x_len - 1 - x);
        (1..=range)
            .map(|r| (x + 1 - r, x + r))
            .cartesian_product(0..y_len)
            .all(|(pair, y)| map.get(&(pair.0, y)) == map.get(&(pair.1, y)))
    });
    let y = (0..y_len - 1).find(|y| {
        let range = cmp::min(y + 1, y_len - 1 - y);
        (1..=range)
            .map(|r| (y + 1 - r, y + r))
            .cartesian_product(0..x_len)
            .all(|(pair, x)| map.get(&(x, pair.0)) == map.get(&(x, pair.1)))
    });
    (x, y)
}

fn find2((x_len, y_len): Coord, map: &HashMap<Coord, u8>) -> (Option<usize>, Option<usize>) {
    let x = (0..x_len - 1).find(|x| {
        let range = cmp::min(x + 1, x_len - 1 - x);
        (1..=range)
            .map(|r| (x + 1 - r, x + r))
            .cartesian_product(0..y_len)
            .filter(|(pair, y)| map.get(&(pair.0, *y)) != map.get(&(pair.1, *y)))
            .count()
            == 1
    });
    let y = (0..y_len - 1).find(|y| {
        let range = cmp::min(y + 1, y_len - 1 - y);
        (1..=range)
            .map(|r| (y + 1 - r, y + r))
            .cartesian_product(0..x_len)
            .filter(|(pair, x)| map.get(&(*x, pair.0)) != map.get(&(*x, pair.1)))
            .count()
            == 1
    });
    (x, y)
}

fn parse<'a>(
    input: &'a str,
    re: &'a Regex,
) -> impl Iterator<Item = (Coord, HashMap<Coord, u8>)> + 'a {
    re.split(input).map(parse_chunk)
}

fn parse_chunk(chunk: &str) -> (Coord, HashMap<Coord, u8>) {
    let (lens, it) = utils::parse_with_lens(chunk, &identity);
    (lens, it.collect())
}

type FindFn = fn(Coord, &HashMap<Coord, u8>) -> (Option<usize>, Option<usize>);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 405);
        assert_eq!(p1(INPUT), 35210);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 400);
        assert_eq!(p2(INPUT), 31974);
    }
}

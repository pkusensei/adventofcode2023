#![allow(dead_code)]

use std::collections::BTreeMap; // avoid Hash stuff

use utils::Coord;

fn p1(input: &str) -> usize {
    let ((x_len, y_len), rocks) = parse(input);
    score(y_len, &tilt_north((x_len, y_len), rocks))
}

fn p2(input: &str) -> usize {
    const COUNT: usize = 1000000000;
    let (lens, mut rocks) = parse(input);
    let mut states = BTreeMap::new();
    let mut i = 0;
    while i < COUNT {
        if let Some(&start) = states.get(&rocks) {
            let loop_size = i - start;
            let remainder = (COUNT - i) % loop_size;
            i = COUNT - remainder;
        } else {
            states.insert(rocks.clone(), i);
        }
        rocks = cycle(lens, rocks);
        i += 1;
    }
    score(lens.1, &rocks)
}

fn score(y_len: usize, rocks: &BTreeMap<Coord, Kind>) -> usize {
    rocks
        .iter()
        .filter_map(|(c, k)| {
            if *k == Kind::Round {
                Some(y_len - c.1)
            } else {
                None
            }
        })
        .sum()
}

fn cycle(lens: Coord, rocks: BTreeMap<Coord, Kind>) -> BTreeMap<Coord, Kind> {
    tilt_east(
        lens,
        tilt_south(lens, tilt_west(lens, tilt_north(lens, rocks))),
    )
}

fn tilt_north((x_len, y_len): Coord, rocks: BTreeMap<Coord, Kind>) -> BTreeMap<Coord, Kind> {
    (0..x_len)
        .flat_map(|x| {
            let v = (0..y_len).map(|y| rocks[&(x, y)]).collect::<Vec<_>>();
            tilt(&v)
                .into_iter()
                .enumerate()
                .map(move |(i, k)| ((x, i), k))
        })
        .collect()
}

fn tilt_south((x_len, y_len): Coord, rocks: BTreeMap<Coord, Kind>) -> BTreeMap<Coord, Kind> {
    (0..x_len)
        .flat_map(|x| {
            let v = (0..y_len).rev().map(|y| rocks[&(x, y)]).collect::<Vec<_>>();
            tilt(&v)
                .into_iter()
                .enumerate()
                .map(move |(i, k)| ((x, y_len - 1 - i), k))
        })
        .collect()
}

fn tilt_west((x_len, y_len): Coord, rocks: BTreeMap<Coord, Kind>) -> BTreeMap<Coord, Kind> {
    (0..y_len)
        .flat_map(|y| {
            let v = (0..x_len).map(|x| rocks[&(x, y)]).collect::<Vec<_>>();
            tilt(&v)
                .into_iter()
                .enumerate()
                .map(move |(i, k)| ((i, y), k))
        })
        .collect()
}

fn tilt_east((x_len, y_len): Coord, rocks: BTreeMap<Coord, Kind>) -> BTreeMap<Coord, Kind> {
    (0..y_len)
        .flat_map(|y| {
            let v = (0..x_len).rev().map(|x| rocks[&(x, y)]).collect::<Vec<_>>();
            tilt(&v)
                .into_iter()
                .enumerate()
                .map(move |(i, k)| ((x_len - 1 - i, y), k))
        })
        .collect()
}

fn tilt(before: &[Kind]) -> Vec<Kind> {
    let mut after = vec![Kind::Empty; before.len()];
    let mut aidx = 0;
    for (i, k) in before.iter().enumerate() {
        match k {
            Kind::Round => {
                after[aidx] = Kind::Round;
                aidx += 1
            }
            Kind::Cubed => {
                after[i] = Kind::Cubed;
                aidx = i + 1
            }
            Kind::Empty => (),
        }
    }
    after
}

fn parse(input: &str) -> (Coord, BTreeMap<Coord, Kind>) {
    let (lens, it) = utils::parse_with_lens(input, &Kind::from);
    (lens, it.collect())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Round,
    Cubed,
    Empty,
}

impl From<u8> for Kind {
    fn from(value: u8) -> Self {
        match value {
            b'O' => Kind::Round,
            b'#' => Kind::Cubed,
            b'.' => Kind::Empty,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#...."#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 136);
        assert_eq!(p1(INPUT), 109939);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 64);
        assert_eq!(p2(INPUT), 101010);
    }
}

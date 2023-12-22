#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::identity,
};

use itertools::Itertools;
use rayon::prelude::*;

use utils::{Coord, Dir};

fn p1(input: &str) -> usize {
    let (lens, grid) = parse(input);
    let start = Beam {
        coord: (0, 0),
        dir: Dir::East,
    };
    bfs(start, lens, &grid)
}

fn p2(input: &str) -> usize {
    let ((x_len, y_len), grid) = parse(input);
    (0..x_len)
        .into_par_iter()
        .map(|x| Beam {
            coord: (x, 0),
            dir: Dir::South,
        })
        .chain((0..x_len).into_par_iter().map(|x| Beam {
            coord: (x, y_len - 1),
            dir: Dir::North,
        }))
        .chain((0..y_len).into_par_iter().map(|y| Beam {
            coord: (0, y),
            dir: Dir::East,
        }))
        .chain((0..y_len).into_par_iter().map(|y| Beam {
            coord: (x_len - 1, y),
            dir: Dir::West,
        }))
        .map(|b| bfs(b, (x_len, y_len), &grid))
        .max()
        .unwrap()
}

fn bfs(start: Beam, lens: Coord, grid: &HashMap<Coord, u8>) -> usize {
    let dir = grid[&start.coord];
    let start = start.change_dir(dir);

    let mut seen = HashSet::new();
    let mut queue = VecDeque::from(start);
    while let Some(curr) = queue.pop_front() {
        if seen.contains(&curr) {
            continue;
        }
        seen.insert(curr);
        if let Some(beam) = curr.proceed(lens) {
            queue.extend(beam.change_dir(grid[&beam.coord]));
        }
    }
    seen.into_iter().unique_by(|a| a.coord).count()
}

fn parse(input: &str) -> (Coord, HashMap<Coord, u8>) {
    let (lens, it) = utils::parse_with_lens(input, &identity);
    (lens, it.collect())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Beam {
    coord: Coord,
    dir: Dir,
}

impl Beam {
    fn change_dir(mut self, alter: u8) -> Vec<Beam> {
        let dirs = match (self.dir, alter) {
            (d, b'.') => vec![d],
            (Dir::North, b'\\') | (Dir::South, b'/') => vec![Dir::West],
            (Dir::North, b'/') | (Dir::South, b'\\') => vec![Dir::East],
            (Dir::West, b'\\') | (Dir::East, b'/') => vec![Dir::North],
            (Dir::West, b'/') | (Dir::East, b'\\') => vec![Dir::South],
            (d @ (Dir::North | Dir::South), b'|') => vec![d],
            (Dir::North | Dir::South, b'-') => vec![Dir::West, Dir::East],
            (d @ (Dir::West | Dir::East), b'-') => vec![d],
            (Dir::West | Dir::East, b'|') => vec![Dir::North, Dir::South],
            _ => unreachable!(),
        };
        dirs.into_iter()
            .map(|d| {
                self.dir = d;
                self
            })
            .collect()
    }

    fn proceed(mut self, (x_len, y_len): Coord) -> Option<Self> {
        match self.dir {
            Dir::North => {
                if self.coord.1 > 0 {
                    self.coord.1 -= 1;
                    Some(self)
                } else {
                    None
                }
            }
            Dir::South => {
                if self.coord.1 < y_len - 1 {
                    self.coord.1 += 1;
                    Some(self)
                } else {
                    None
                }
            }
            Dir::West => {
                if self.coord.0 > 0 {
                    self.coord.0 -= 1;
                    Some(self)
                } else {
                    None
                }
            }
            Dir::East => {
                if self.coord.0 < x_len - 1 {
                    self.coord.0 += 1;
                    Some(self)
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|...."#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 46);
        assert_eq!(p1(INPUT), 7434);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 51);
        assert_eq!(p2(INPUT), 8183);
    }
}

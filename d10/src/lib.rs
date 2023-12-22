#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::identity,
};

use utils::{Coord, Dir};

fn p1(input: &str) -> usize {
    let (start, map) = parse(input);
    find_loop(start, &map).len() / 2
}

fn p2(input: &str) -> usize {
    let (start, map) = parse(input);
    let main_loop = find_loop(start, &map);
    let mut count = 0;
    for (y, line) in input.lines().enumerate() {
        for x in 0..line.len() {
            let coord = (x + 1, y + 1);
            if main_loop.contains(&coord) {
                continue;
            }
            // Cast a ray from left to right
            // Count its crossings that points up/north
            // '|' as 1
            // 'L--7' as 1 i.e still in loop
            // 'L--J' as 2 i.e out of loop
            // 'F--J' as 1
            // 'F--7' as 0
            let cross = (1..coord.0)
                .filter_map(|v| {
                    if main_loop.contains(&(v, coord.1)) {
                        map.get(&(v, coord.1))
                    } else {
                        None
                    }
                })
                .filter(|b| b.contains(&Dir::North))
                .count();
            if cross & 1 == 1 {
                count += 1;
            }
        }
    }
    count
}

fn find_loop(start: Coord, map: &HashMap<Coord, [Dir; 2]>) -> HashSet<Coord> {
    let mut seen = HashSet::new();
    let mut pipes = VecDeque::from([start]);
    while let Some(pipe) = pipes.pop_front() {
        if seen.contains(&pipe) {
            continue;
        }
        seen.insert(pipe);
        for n in map[&pipe].map(|dir| neighbor(pipe, dir)) {
            if map.contains_key(&n) {
                // this if cond is useless
                pipes.push_back(n)
            }
        }
    }
    seen
}

const fn neighbor(coord: Coord, dir: Dir) -> Coord {
    match dir {
        Dir::North => (coord.0, coord.1 - 1),
        Dir::South => (coord.0, coord.1 + 1),
        Dir::East => (coord.0 + 1, coord.1),
        Dir::West => (coord.0 - 1, coord.1),
    }
}

fn parse(input: &str) -> (Coord, HashMap<Coord, [Dir; 2]>) {
    let (_, it) = utils::parse_with_lens(input, &identity);
    let mut map = HashMap::new();
    let mut start = (0, 0);
    for (c, byte) in it {
        let coord = (c.0 + 1, c.1 + 1);
        if byte == b'S' {
            start = coord;
            map.insert(coord, dirs(b'F'));
        } else if byte != b'.' {
            map.insert(coord, dirs(byte));
        }
    }
    (start, map)
}

const fn dirs(shape: u8) -> [Dir; 2] {
    match shape {
        b'|' => [Dir::North, Dir::South],
        b'-' => [Dir::East, Dir::West],
        b'L' => [Dir::North, Dir::East],
        b'J' => [Dir::North, Dir::West],
        b'7' => [Dir::South, Dir::West],
        b'F' => [Dir::South, Dir::East],
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"-L|F7
    7S-7|
    L|7||
    -L-J|
    L|-JF"#;

    const TEST2: &'static str = r#"..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ..."#;

    const TEST3: &'static str = r#"...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ..........."#;

    const TEST4: &'static str = r#".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ..."#;

    const TEST5: &'static str = r#"FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 4);
        assert_eq!(p1(TEST2), 8);
        assert_eq!(p1(INPUT), 6838);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST3), 4);
        assert_eq!(p2(TEST4), 8);
        assert_eq!(p2(TEST5), 10);
        assert_eq!(p2(INPUT), 451);
    }
}

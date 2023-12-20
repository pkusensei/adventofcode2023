#![allow(dead_code)]

use std::collections::HashMap;

use regex::Regex;

fn p1(input: &str) -> usize {
    let (inst, map) = parse(input);
    solve(inst, &map, "AAA", |s| s == "ZZZ")
}

fn p2(input: &str) -> usize {
    let (inst, map) = parse(input);
    map.keys()
        .filter_map(|s| {
            if s.ends_with('A') {
                Some(solve(inst, &map, s, |end| end.ends_with('Z')))
            } else {
                None
            }
        })
        .fold(1, utils::lcm) // reduce(lcm).unwrap()
}

fn solve(
    inst: &str,
    map: &HashMap<&str, (&str, &str)>,
    start: &str,
    end: fn(&str) -> bool,
) -> usize {
    let mut curr = start;
    inst.bytes()
        .cycle()
        .enumerate()
        .find_map(|(idx, byte)| {
            curr = if byte == b'L' {
                map[curr].0
            } else {
                map[curr].1
            };
            if end(curr) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .unwrap()
}

fn parse(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let inst = lines.next().unwrap();
    // Wow this thing kills performance
    let re = Regex::new(r#"(\w+)"#).unwrap();

    let map = lines
        .skip(1)
        .map(|line| {
            let mut it = re.find_iter(line).map(|m| m.as_str());
            (it.next().unwrap(), (it.next().unwrap(), it.next().unwrap()))
        })
        .collect();
    (inst, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const TEST2: &'static str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const TEST3: &'static str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 2);
        assert_eq!(p1(TEST2), 6);
        assert_eq!(p1(INPUT), 16579);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST3), 6);
        assert_eq!(p2(INPUT), 12927600769609);
    }
}

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

fn p1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .filter_map(|count| {
            if count > 0 {
                Some(2u32.pow(count as u32 - 1))
            } else {
                None
            }
        })
        .sum()
}

fn p2(input: &str) -> usize {
    let mut cards: HashMap<_, _> = input.lines().enumerate().map(|(i, _)| (i, 1)).collect();
    for (idx, count) in input.lines().map(parse_line).enumerate() {
        let curr = *cards.get(&idx).unwrap();
        for i in 0..count {
            cards.get_mut(&(idx + 1 + i)).map(|value| *value += curr);
        }
    }
    cards.values().sum()
}

fn parse_line(line: &str) -> usize {
    let (wins, nums) = line
        .split_once(':')
        .map(|(_, s)| s.split_once('|'))
        .flatten()
        .unwrap();
    let wins: HashSet<u32> = wins
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let nums = nums
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    wins.intersection(&nums).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 13);
        assert_eq!(p1(INPUT), 18519);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 30);
        assert_eq!(p2(INPUT), 11787590);
    }
}

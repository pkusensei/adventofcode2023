#![allow(dead_code)]

use itertools::Itertools;

const CARDS: &[u8; 13] = b"AKQJT98765432";

const CARDS2: &[u8; 13] = b"AKQT98765432J";

fn p1(input: &str) -> usize {
    solve(input, parse_line)
}

fn p2(input: &str) -> usize {
    solve(input, |line| parse_line(line).update())
}

fn solve(input: &str, parse: fn(&str) -> Hand) -> usize {
    input
        .lines()
        .map(parse)
        .sorted()
        .enumerate()
        .map(|(i, c)| (i + 1) * c.bid)
        .sum()
}

fn parse_line(line: &str) -> Hand {
    let mut it = line.split_whitespace();
    let cards: [u8; 5] = it.next().unwrap().as_bytes().to_vec().try_into().unwrap();
    let bid = it.next().and_then(|s| s.parse().ok()).unwrap();
    Hand {
        cards,
        bid,
        kind: Kind::new(&cards),
        lst: CARDS,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Kind {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    High,
}

impl Kind {
    fn new(cards: &[u8]) -> Self {
        let counts = cards
            .iter()
            .counts()
            .values()
            .sorted()
            .cloned()
            .collect::<Vec<_>>();
        match counts[..] {
            [5] => Kind::Five,
            [1, 4] => Kind::Four,
            [2, 3] => Kind::FullHouse,
            [1, 1, 3] => Kind::Three,
            [1, 2, 2] => Kind::TwoPair,
            [1, 1, 1, 2] => Kind::OnePair,
            [1, .., 1] => Kind::High,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [u8; 5],
    bid: usize,
    kind: Kind,
    lst: &'static [u8; 13],
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind == other.kind {
            self.cards
                .into_iter()
                .zip(other.cards)
                .find_map(|(left, right)| {
                    let l = self.lst.iter().position(|c| *c == left);
                    let r = self.lst.iter().position(|c| *c == right);
                    match l.partial_cmp(&r) {
                        Some(std::cmp::Ordering::Less) => Some(std::cmp::Ordering::Greater),
                        Some(std::cmp::Ordering::Greater) => Some(std::cmp::Ordering::Less),
                        _ => None, // skips l==r case
                    }
                })
        } else if self.kind < other.kind {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Hand {
    fn update(mut self) -> Self {
        if self.cards.contains(&b'J') {
            let freq = self
                .cards
                .iter()
                .filter(|b| **b != b'J')
                .cloned()
                .counts()
                .into_iter()
                .max_by_key(|(_k, c)| *c)
                .map(|(k, _c)| k)
                .unwrap_or(b'J'); // JJJJJ
            let cards = self
                .cards
                .iter()
                .map(|b| if *b == freq { b'J' } else { *b })
                .collect::<Vec<_>>();
            self.kind = Kind::new(&cards);
        }
        self.lst = CARDS2;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 6440);
        assert_eq!(p1(INPUT), 250898830);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 5905);
        assert_eq!(p2(INPUT), 252127335);
    }
}

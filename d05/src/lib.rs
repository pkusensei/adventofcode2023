#![allow(dead_code)]

use itertools::Itertools;

fn p1(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds = parse_seeds(lines.next().unwrap());
    let groups = parse_chunks(lines);

    seeds
        .map(|seed| {
            let mut val = seed;
            for group in groups.iter() {
                val = group
                    .iter()
                    .find_map(|r| {
                        if r.src <= val && val < r.src + r.rng {
                            Some(r.dst + (val - r.src))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(val);
            }
            val
        })
        .min()
        .unwrap()
}

fn p2(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut seeds: Vec<_> = parse_seeds(lines.next().unwrap())
        .tuples()
        .map(|(st, rng)| Seeds::new(st, rng))
        .collect();
    let groups = parse_chunks(lines);

    for group in groups.iter() {
        seeds = seeds.into_iter().flat_map(|s| s.split(group)).collect();
        seeds = seeds.into_iter().map(|s| s.update(group)).collect();
    }
    seeds.into_iter().min_by_key(|s| s.st).unwrap().st
}

fn parse_chunks(lines: std::str::Lines<'_>) -> Vec<Vec<Range>> {
    let mut groups = vec![];
    let mut group: Vec<Range> = vec![];

    for line in lines {
        if line.trim().is_empty() {
            continue;
        } else if line.contains("map") {
            group.sort_by_key(|r| r.src);
            groups.push(group);
            group = vec![];
        } else {
            group.push(parse_range(line))
        }
    }
    group.sort_by_key(|r| r.src);
    groups.push(group);
    groups
}

fn parse_seeds(line: &str) -> impl Iterator<Item = u64> + '_ {
    line.split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
}

fn parse_range(line: &str) -> Range {
    let (dst, src, rng) = line
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Range { dst, src, rng }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    dst: u64,
    src: u64,
    rng: u64,
}

#[derive(Debug, Clone, Copy)]
struct Seeds {
    st: u64,
    rng: u64,
}

impl Seeds {
    const fn new(st: u64, rng: u64) -> Self {
        Seeds { st, rng }
    }

    fn split(self, ranges: &[Range]) -> Vec<Self> {
        let mut res = vec![];
        let mut curr = self;
        for range in ranges {
            let mut v = curr.split_range(range);
            curr = v.pop().unwrap();
            res.extend(v.into_iter())
        }
        // Can't believe I was debating whether there should be an if condition on this
        // e.g if curr == self
        // OR
        // if curr != self
        res.push(curr);
        res
    }

    fn split_range(self, range: &Range) -> Vec<Self> {
        let mut res = vec![];
        if let Some((left, right)) = self.split_at(range.src) {
            res.push(left);
            if let Some((_mid, last)) = right.split_at(range.src + range.rng - 1) {
                // 1, 2, 3, 4, 5
                //       3, 4
                res.push(last);
                res.push(Self::new(range.src, range.rng))
            } else {
                // 1, 2, 3, 4, 5
                //       3, 4, 5, 6
                res.push(right)
            }
        } else if let Some((left, right)) = self.split_at(range.src + range.rng - 1) {
            res.push(right);
            if let Some((first, _mid)) = left.split_at(range.src) {
                // 1, 2, 3, 4, 5
                //       3, 4
                res.push(first);
                res.push(Self::new(range.src, range.rng))
            } else {
                //    2, 3, 4, 5
                // 1, 2, 3
                res.push(left)
            }
        } else {
            res.push(self);
        }

        res.sort_by_key(|s| s.st);
        res
    }

    const fn split_at(self, point: u64) -> Option<(Self, Self)> {
        if self.st < point && self.st + self.rng > point {
            // 1, 2, 3
            //    2
            Some((
                Seeds::new(self.st, point - self.st),
                Seeds::new(point, self.rng + self.st - point),
            ))
        } else if self.st == point {
            // 1, 2, 3
            // 1
            Some((
                Seeds::new(self.st, 1),
                Seeds::new(self.st + 1, self.rng - 1), // what is self is a single point
            ))
        } else if self.st + self.rng - 1 == point {
            // 1, 2, 3
            //       3
            Some((
                Seeds::new(self.st, self.rng - 1),
                Seeds::new(self.st + self.rng - 1, 1),
            ))
        } else {
            None
        }
    }

    fn update(self, ranges: &[Range]) -> Self {
        ranges
            .iter()
            .find_map(|r| self.update_range(r))
            .unwrap_or(self)
    }

    const fn update_range(self, range: &Range) -> Option<Self> {
        //    2
        // 1, 2, 3
        if range.src <= self.st && self.st + self.rng <= range.src + range.rng {
            Some(Self::new(range.dst + self.st - range.src, self.rng))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 35);
        assert_eq!(p1(INPUT), 282277027);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 46);
        assert_eq!(p2(INPUT), 11554135);
    }
}

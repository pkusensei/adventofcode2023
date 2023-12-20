#![allow(dead_code)]

use std::collections::HashMap;

use regex::Regex;

fn p1(input: &str) -> u32 {
    let empty = Regex::new(r#"(?m)^\s*$\n?"#).unwrap();
    let mut chunks = empty.split(input);
    let rules = parse(chunks.next().unwrap());

    let mut accepted = vec![];
    let partex = Regex::new(r#"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}"#).unwrap();
    for part in partex.captures_iter(chunks.next().unwrap()).map(|c| {
        let (_, nums) = c.extract::<4>();
        parse_part(nums)
    }) {
        let mut curr = "in";
        while curr != "A" && curr != "R" {
            let rule = &rules[curr];
            for r in rule {
                if let Some(dest) = r.run(&part) {
                    curr = dest;
                    break;
                }
            }
        }
        if curr == "A" {
            accepted.push(part)
        }
    }
    accepted.into_iter().map(|p| p.values().sum::<u32>()).sum()
}

fn p2(input: &str) -> usize {
    let empty = Regex::new(r#"(?m)^\s*$\n?"#).unwrap();
    let lines = empty.split(input).next().unwrap();
    let rules = parse(lines);

    let start = HashMap::from(b"xmas".map(|b| (b, (1, 4000))));
    solve("in", start, &rules)
}

fn solve(curr: &str, mut ratings: Ratings, rules: &HashMap<&str, Vec<Rule<'_>>>) -> usize {
    if curr == "A" {
        return ratings.values().map(|r| (r.1 - r.0 + 1) as usize).product();
    } else if curr == "R" {
        return 0;
    }

    let mut res = 0;
    for r in &rules[&curr] {
        for (dest, rat) in r.split(&ratings) {
            if let Some(dest) = dest {
                res += solve(dest, rat, rules);
            } else {
                ratings = rat
            }
        }
    }
    res
}

fn parse(lines: &str) -> HashMap<&str, Vec<Rule<'_>>> {
    let linex = Regex::new(r#"(\w+)\{(.+,*)+}"#).unwrap();
    let rulex = Regex::new(r#"(\w)([<|>])(\d+):(\w+)"#).unwrap();
    linex
        .captures_iter(lines)
        .map(|c| {
            let (_, group) = c.extract::<2>();
            parse_rule(group, &rulex)
        })
        .collect()
}

fn parse_rule<'a>([start, line]: [&'a str; 2], re: &Regex) -> (&'a str, Vec<Rule<'a>>) {
    let last = line.split(',').last().unwrap();
    let rules = re
        .captures_iter(line)
        .map(|cap| {
            let (_, [part, op, num, dest]) = cap.extract::<4>();
            Rule {
                selector: part.as_bytes()[0],
                op: op.as_bytes()[0],
                target: num.parse().unwrap(),
                dest,
            }
        })
        .chain([Rule {
            selector: 0,
            op: 0,
            target: 0,
            dest: last,
        }])
        .collect();

    (start, rules)
}

fn parse_part(nums: [&str; 4]) -> Part {
    b"xmas"
        .iter()
        .zip(nums)
        .map(|(&b, s)| (b, s.parse().unwrap()))
        .collect()
}

type Part = HashMap<u8, u32>;
type Ratings = HashMap<u8, (u32, u32)>;

#[derive(Debug, Clone, Copy)]
struct Rule<'a> {
    selector: u8, // 'x', 'm', 'a', 's'
    op: u8,       // '<', '>'
    target: u32,
    dest: &'a str,
}

impl<'a> Rule<'a> {
    fn run(&self, part: &Part) -> Option<&'a str> {
        if self.selector == 0 {
            Some(self.dest)
        } else {
            let value = part[&self.selector];
            if (self.op == b'<' && value < self.target) || (self.op == b'>' && value > self.target)
            {
                Some(self.dest)
            } else {
                None
            }
        }
    }

    fn split(&self, ratings: &Ratings) -> Vec<(Option<&'a str>, Ratings)> {
        if self.selector == 0 {
            vec![(Some(self.dest), ratings.clone())]
        } else {
            let (vmin, vmax) = ratings[&self.selector];
            let rs = if self.op == b'<' {
                if vmin < self.target && self.target <= vmax {
                    let r1 = (vmin, self.target - 1);
                    let r2 = (self.target, vmax);
                    vec![(Some(self.dest), r1), (None, r2)]
                } else if self.target <= vmin {
                    vec![(None, (vmin, vmax))]
                } else {
                    vec![(Some(self.dest), (vmin, vmax))]
                }
            } else if self.op == b'>' {
                if vmin <= self.target && self.target < vmax {
                    let r1 = (vmin, self.target);
                    let r2 = (self.target + 1, vmax);
                    vec![(None, r1), (Some(self.dest), r2)]
                } else if self.target < vmin {
                    vec![(Some(self.dest), (vmin, vmax))]
                } else {
                    vec![(None, (vmin, vmax))]
                }
            } else {
                unreachable!()
            };
            rs.into_iter()
                .map(|(dest, r)| {
                    let mut rat = ratings.clone();
                    if let Some(v) = rat.get_mut(&self.selector) {
                        *v = r
                    };
                    (dest, rat)
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}
    
    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 19114);
        assert_eq!(p1(INPUT), 332145);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 167409079868000);
        assert_eq!(p2(INPUT), 136661579897555);
    }
}

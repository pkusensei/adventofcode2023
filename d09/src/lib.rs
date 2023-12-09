#![allow(dead_code)]

fn p1(input: &str) -> i64 {
    solve(input, true)
}

fn p2(input: &str) -> i64 {
    solve(input, false)
}

fn solve(input: &str, p1: bool) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums = parse_line(line).collect::<Vec<_>>();
            calc(&nums, p1)
        })
        .sum()
}

fn calc(nums: &[i64], p1: bool) -> i64 {
    let delta = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(l, r)| r - l)
        .collect::<Vec<_>>();
    if p1 {
        if delta.iter().all(|n| *n == 0) {
            *nums.last().unwrap()
        } else {
            nums.last().unwrap() + calc(&delta, p1)
        }
    } else {
        if delta.iter().all(|n| *n == 0) {
            *nums.first().unwrap()
        } else {
            nums.first().unwrap() - calc(&delta, p1)
        }
    }
}

fn parse_line(line: &str) -> impl Iterator<Item = i64> + '_ {
    line.split_whitespace().map(|s| s.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 114);
        assert_eq!(p1(INPUT), 1681758908);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 2);
        assert_eq!(p2(INPUT), 803);
    }
}

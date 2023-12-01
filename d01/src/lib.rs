#![allow(dead_code)]

fn solve(input: &str, parse: fn(&str) -> u32) -> u32 {
    input.lines().map(parse).sum()
}

fn parse1(line: &str) -> u32 {
    let mut it = line.chars().filter_map(|c| c.to_digit(10));
    let mut rit = it.clone().rev();
    it.next().unwrap() * 10 + rit.next().unwrap()
}

fn parse2(line: &str) -> u32 {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let it = WORDS
        .into_iter()
        .enumerate()
        .chain(DIGITS.into_iter().enumerate());
    let left = it
        .clone()
        .filter_map(|(num, s)| line.find(s).map(|i| (i, num as u32 + 1)))
        .min_by_key(|(i, _n)| *i)
        .map(|(_, n)| n)
        .unwrap();
    let right = it
        .filter_map(|(num, s)| line.rfind(s).map(|i| (i, num as u32 + 1)))
        .max_by_key(|(i, _n)| *i)
        .map(|(_, n)| n)
        .unwrap();

    left * 10 + right
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &'static str = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;

    const TEST2: &'static str = r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(solve(TEST1, parse1), 142);
        assert_eq!(solve(INPUT, parse1), 55172)
    }

    #[test]
    fn test2() {
        assert_eq!(solve(TEST2, parse2), 281);
        assert_eq!(solve(INPUT, parse2), 54925)
    }
}

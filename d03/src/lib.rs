#![allow(dead_code)]

use std::collections::HashMap;

use itertools::Itertools;

fn p1(input: &str) -> u32 {
    let (numbers, symbols, gears) = parse(input);
    numbers
        .into_iter()
        .filter_map(|num| {
            if num
                .neighbors()
                .any(|pos| symbols.contains(&pos) || gears.contains(&pos))
            {
                Some(num.num)
            } else {
                None
            }
        })
        .sum()
}

fn p2(input: &str) -> u32 {
    let (numbers, _, gears) = parse(input);
    let mut gears: HashMap<(i32, i32), Vec<u32>> =
        gears.into_iter().map(|pos| (pos, vec![])).collect();
    for number in numbers.into_iter() {
        for n in number.neighbors() {
            if let Some(v) = gears.get_mut(&n) {
                v.push(number.num)
            }
        }
    }
    gears
        .into_iter()
        .filter_map(|(_k, v)| {
            if v.len() == 2 {
                Some(v[0] * v[1])
            } else {
                None
            }
        })
        .sum()
}

fn parse(input: &str) -> (Vec<Number>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut gears = vec![];
    for (y, line) in input.lines().enumerate() {
        let (n, s, g) = parse_line(line, y as i32);
        numbers.extend(n.into_iter());
        symbols.extend(s.into_iter());
        gears.extend(g.into_iter());
    }
    (numbers, symbols, gears)
}

fn parse_line(line: &str, y: i32) -> (Vec<Number>, Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut gears = vec![];
    let mut x0 = 0;
    let mut num = String::new();
    for (x, c) in line.chars().enumerate() {
        match c {
            '.' => {
                if !num.is_empty() {
                    numbers.push(Number {
                        x0,
                        x1: x as i32 - 1,
                        y,
                        num: num.parse().unwrap(),
                    });
                    num.clear()
                }
            }
            '*' => {
                if !num.is_empty() {
                    numbers.push(Number {
                        x0,
                        x1: x as i32 - 1,
                        y,
                        num: num.parse().unwrap(),
                    });
                    num.clear()
                }
                gears.push((x as i32, y));
            }
            '0'..='9' => {
                if num.is_empty() {
                    x0 = x as i32
                }
                num.push(c)
            }
            _ => {
                if !num.is_empty() {
                    numbers.push(Number {
                        x0,
                        x1: x as i32 - 1,
                        y,
                        num: num.parse().unwrap(),
                    });
                    num.clear()
                }
                symbols.push((x as i32, y));
            }
        }
    }
    if !num.is_empty() {
        numbers.push(Number {
            x0,
            x1: line.len() as i32 - 1, // Well chars utf-8 shenanigans
            y,
            num: num.parse().unwrap(),
        });
    }
    (numbers, symbols, gears)
}

#[derive(Copy, Clone)]
struct Number {
    x0: i32,
    x1: i32,
    y: i32,
    num: u32,
}

impl Number {
    fn neighbors(self) -> impl Iterator<Item = (i32, i32)> {
        ((self.x0 - 1)..=(self.x1 + 1))
            .cartesian_product((self.y - 1)..=(self.y + 1))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .filter(move |(x, y)| *y != self.y || !(self.x0..=self.x1).contains(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 4361);
        assert_eq!(p1(INPUT), 521515);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 467835);
        assert_eq!(p2(INPUT), 69527306);
    }
}

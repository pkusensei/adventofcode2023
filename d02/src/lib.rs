#![allow(dead_code)]

fn p1(input: &str) -> usize {
    parse(input)
        .filter_map(|(idx, r, g, b)| {
            if r <= 12 && g <= 13 && b <= 14 {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum()
}

fn p2(input: &str) -> u32 {
    parse(input).map(|(_, r, g, b)| r * g * b).sum()
}

fn parse(input: &str) -> impl Iterator<Item = (usize, u32, u32, u32)> + '_ {
    input.lines().enumerate().map(|(idx, line)| {
        let nums = parse_line(line);
        let r = nums.clone().map(|x| x.0).max().unwrap();
        let g = nums.clone().map(|x| x.1).max().unwrap();
        let b = nums.map(|x| x.2).max().unwrap();
        (idx, r, g, b)
    })
}

fn parse_line(line: &str) -> impl Iterator<Item = (u32, u32, u32)> + Clone + '_ {
    line.split_once(':').unwrap().1.split(';').map(|take| {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for pair in take.split(',') {
            let mut s = pair.split_whitespace();
            let num = s.next().unwrap().parse().unwrap();
            match s.next().unwrap() {
                "red" => r = num,
                "green" => g = num,
                "blue" => b = num,
                _ => unreachable!(),
            }
        }
        (r, g, b)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 8);
        assert_eq!(p1(INPUT), 3059);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 2286);
        assert_eq!(p2(INPUT), 65371);
    }
}

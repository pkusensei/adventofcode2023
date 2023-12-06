#![allow(dead_code)]

fn p1(input: &str) -> usize {
    let mut it = input.lines();
    let tm = it.next().map(parse_line).unwrap();
    let dst = it.next().map(parse_line).unwrap();
    tm.zip(dst).filter_map(|(tm, dst)| count(tm, dst)).product()
}

fn p2(input: &str) -> usize {
    let mut it = input.lines();
    let tm = it.next().map(parse_line).unwrap().collect::<String>();
    let dst = it.next().map(parse_line).unwrap().collect::<String>();
    count(&tm, &dst).unwrap_or(0)
}

fn count(tm: &str, dst: &str) -> Option<usize> {
    let tm: usize = tm.parse().unwrap();
    let dst: usize = dst.parse().unwrap();
    let count = if tm & 1 == 1 {
        // odd time
        let mid = (tm - 1) / 2;
        (1..=mid)
            .rev()
            .filter(|trial| trial * (tm - trial) > dst)
            .count()
            * 2
    } else {
        let mid = tm / 2;
        let c = (1..=mid)
            .rev()
            .filter(|trial| trial * (tm - trial) > dst)
            .count()
            * 2;
        if c > 0 {
            c - 1
        } else {
            0
        }
    };
    if count > 0 {
        Some(count)
    } else {
        None
    }
}

fn parse_line(line: &str) -> impl Iterator<Item = &str> {
    line.split_once(':').unwrap().1.split_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"Time:      7  15   30
    Distance:  9  40  200"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 288);
        assert_eq!(p1(INPUT), 1108800);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 71503);
        assert_eq!(p2(INPUT), 36919753);
    }
}

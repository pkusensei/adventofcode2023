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
    fn ct(mid: usize, tm: usize, dst: usize) -> usize {
        (1..=mid)
            .rev()
            .take_while(|trial| trial * (tm - trial) > dst)
            .count()
            * 2
        // OR
        // - trial^2 + tm*trial - dst = 0
        // a = -1, b = tm, c = - dst
        // sqrt(b^2 - 4 a*c)
        // sqrt = f64::sqrt((tm * tm - 4 * dst) as f64)
        // hi = (tm + sqrt)/2
        // lo = (tm - sqrt)/2
        // hi - lo
    }

    let tm: usize = tm.parse().unwrap();
    let dst: usize = dst.parse().unwrap();
    let count = if tm & 1 == 1 {
        // odd time
        ct((tm - 1) / 2, tm, dst)
    } else {
        let c = ct(tm / 2, tm, dst);
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

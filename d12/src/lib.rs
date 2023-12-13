#![allow(dead_code)]

use std::collections::HashMap;

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (v, p) = parse_line(line);
            let mut cache = HashMap::new();
            solve(&v, &p, &mut cache)
        })
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (v, p) = parse_line(line);
            let mut cache = HashMap::new();
            let values = v
                .iter()
                .cloned()
                .chain([b'?'])
                .cycle()
                .take(v.len() * 5 + 4)
                .collect::<Vec<_>>();
            let pattern = p
                .iter()
                .cloned()
                .cycle()
                .take(p.len() * 5)
                .collect::<Vec<_>>();
            solve(&values, &pattern, &mut cache)
        })
        .sum()
}

fn solve(
    values: &[u8],
    pattern: &[u8],
    cache: &mut HashMap<(Vec<u8>, Vec<u8>), usize>,
) -> usize {
    let key = (values.to_vec(), pattern.to_vec());
    if let Some(v) = cache.get(&key) {
        return *v;
    };
    let res = if values.is_empty() {
        pattern.is_empty().into()
    } else if pattern.is_empty() && values.contains(&b'#') {
        0
    } else if values[0] == b'.' {
        solve(&values[1..], pattern, cache)
    } else if values[0] == b'?' {
        let mut v = values.to_vec();
        v[0] = b'#'; // flip or skip first element
        solve(&v, pattern, cache) + solve(&values[1..], pattern, cache)
    } else if values.len() < pattern[0] as usize {
        // ##.. 5,1 ===> fails
        0
    } else if values.iter().take(pattern[0].into()).any(|b| *b == b'.') {
        // .#..# 2,1 ===> fails
        0
    } else if values.len() == pattern[0].into() {
        // fall thru from above: value is all '#'
        (pattern.len() == 1).into()
    } else if values[pattern[0] as usize] == b'#' {
        // value.len() > pattern[0]
        // ###..# 2,1  ===> fails
        0
    } else {
        solve(&values[pattern[0] as usize + 1..], &pattern[1..], cache)
    };
    cache.insert(key, res);
    res
}

fn parse_line(line: &str) -> (Vec<u8>, Vec<u8>) {
    let mut it = line.split_whitespace();
    let v = it.next().map(|s| s.as_bytes().to_vec()).unwrap();
    let n = it
        .next()
        .and_then(|s| s.split(',').map(|num| num.parse().ok()).collect())
        .unwrap();
    (v, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 21);
        assert_eq!(p1(INPUT), 7674);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 525152);
        assert_eq!(p2(INPUT), 4443895258186);
    }
}

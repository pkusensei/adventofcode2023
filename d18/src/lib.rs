#![allow(dead_code)]

use regex::Regex;

const RE: &str = r#"(\w) (\d+) \(#(\S+)(\d)\)"#;

fn p1(input: &str) -> isize {
    let re = Regex::new(RE).unwrap();
    let mut curr = (0, 0);
    let mut vertices = vec![curr];
    let mut boundary = 1;
    for [dir, steps, ..] in parse(input, &re) {
        let steps = steps.parse().unwrap();
        curr = proceed(curr, dir.as_bytes()[0], steps);
        vertices.push(curr);
        boundary += steps;
    }
    solve(&vertices, boundary)
}

fn p2(input: &str) -> isize {
    let re = Regex::new(RE).unwrap();
    let mut curr = (0, 0);
    let mut vertices = vec![curr];
    let mut boundary = 1;
    for [.., steps, dir] in parse(input, &re) {
        let steps = isize::from_str_radix(steps, 16).unwrap();
        curr = proceed(curr, dir.as_bytes()[0], steps);
        vertices.push(curr);
        boundary += steps;
    }
    solve(&vertices, boundary)
}

fn solve(vertices: &[(isize, isize)], boundary: isize) -> isize {
    // Shoelace formula
    // https://www.themathdoctors.org/polygon-coordinates-and-areas/
    let area: isize = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum();
    // Plug in Pick's theorem
    area.abs() / 2 + boundary / 2 + 1
}

const fn proceed(curr: (isize, isize), dir: u8, steps: isize) -> (isize, isize) {
    match dir {
        b'U' | b'3' => (curr.0, curr.1 - steps),
        b'D' | b'1' => (curr.0, curr.1 + steps),
        b'R' | b'0' => (curr.0 + steps, curr.1),
        b'L' | b'2' => (curr.0 - steps, curr.1),
        _ => unreachable!(),
    }
}

fn parse<'a>(input: &'a str, re: &'a Regex) -> impl Iterator<Item = [&'a str; 4]> {
    re.captures_iter(input).map(|cap| {
        let (_, group) = cap.extract::<4>();
        group
    })
}

fn flood_fill(input: &str) -> isize {
    use std::collections::{HashSet, VecDeque};

    fn proceed_line(curr: (isize, isize), dir: u8, steps: isize) -> Vec<(isize, isize)> {
        match dir {
            b'U' | b'3' => (1..=steps).map(|step| (curr.0, curr.1 - step)).collect(),
            b'D' | b'1' => (1..=steps).map(|step| (curr.0, curr.1 + step)).collect(),
            b'R' | b'0' => (1..=steps).map(|step| (curr.0 + step, curr.1)).collect(),
            b'L' | b'2' => (1..=steps).map(|step| (curr.0 - step, curr.1)).collect(),
            _ => unreachable!(),
        }
    }
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let re = Regex::new(RE).unwrap();
    let mut curr = (0, 0);
    let mut boundary = HashSet::from([curr]);
    for [dir, steps, ..] in parse(input, &re) {
        let line = proceed_line(curr, dir.as_bytes()[0], steps.parse().unwrap());
        curr = *line.last().unwrap();
        boundary.extend(line);
    }
    // Expand the grid out by one
    let xmin = boundary.iter().map(|c| c.0).min().unwrap() - 1;
    let xmax = boundary.iter().map(|c| c.0).max().unwrap() + 1;
    let ymin = boundary.iter().map(|c| c.1).min().unwrap() - 1;
    let ymax = boundary.iter().map(|c| c.1).max().unwrap() + 1;

    // Start from topleft point, which is also outside
    let mut outside = HashSet::from([(xmin, ymin)]);
    let mut queue = VecDeque::from([(xmin, ymin)]);
    while let Some((cur_x, cur_y)) = queue.pop_front() {
        for (dx, dy) in DIRS {
            // Pick one out of four directions
            let (x, y) = (cur_x + dx, cur_y + dy);
            if x < xmin || xmax < x || y < ymin || ymax < y {
                // Falls out of expanded graph
                continue;
            }
            if outside.contains(&(x, y)) || boundary.contains(&(x, y)) {
                // Already seen
                // or
                // Touches boundary; potentially sticks inside
                continue;
            }
            // Still outside
            queue.push_back((x, y));
            outside.insert((x, y));
        }
    }
    (xmax - xmin + 1) * (ymax - ymin + 1) - outside.len() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 62);
        assert_eq!(p1(INPUT), 39039);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 952408144115);
        assert_eq!(p2(INPUT), 44644464596918);
    }

    #[test]
    #[ignore = "Retrofitting flood-fill into Part 1"]
    fn test_flood_fill() {
        assert_eq!(flood_fill(TEST), 62);
        assert_eq!(flood_fill(INPUT), 39039);
    }
}

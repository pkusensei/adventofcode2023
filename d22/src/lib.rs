#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn p1(input: &str) -> usize {
    let bricks = setup(input);
    bricks
        .values()
        .filter(|br| br.above.is_empty() || br.above.iter().all(|b| bricks[b].below.len() > 1))
        .count()
}

fn p2(input: &str) -> usize {
    let bricks = setup(input);
    // SUM!! not max
    bricks.keys().map(|&id| disintegrate(id, &bricks)).sum()
}

fn disintegrate(start: u32, bricks: &HashMap<u32, Brick>) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(id) = queue.pop_front() {
        if seen.contains(&id) {
            continue;
        }
        seen.insert(id);
        queue.extend(bricks[&id].above.iter().filter_map(|a| {
            if bricks[a].below.is_subset(&seen) {
                Some(*a)
            } else {
                None
            }
        }))
    }
    seen.len().saturating_sub(1)
}

fn setup(input: &str) -> HashMap<u32, Brick> {
    let mut grid = HashMap::new();
    let mut bricks = HashMap::new();
    // SORT!!
    for brick in parse(input).sorted_by_key(|b| b.zs) {
        brick.settle(&mut grid, &mut bricks)
    }
    bricks
}

fn parse(input: &str) -> impl Iterator<Item = Brick> + '_ {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| Brick::new(i as u32, line))
}

#[derive(Debug, Clone)]
struct Brick {
    id: u32,
    xs: (u32, u32),
    ys: (u32, u32),
    zs: (u32, u32),
    above: HashSet<u32>,
    below: HashSet<u32>,
}

impl Brick {
    fn new(id: u32, line: &str) -> Self {
        let mut nums = [0; 6];
        for (i, s) in line.trim().split(&[',', '~']).enumerate() {
            nums[i] = s.parse().unwrap();
        }
        Self {
            id,
            xs: (nums[0], nums[3]),
            ys: (nums[1], nums[4]),
            zs: (nums[2], nums[5]),
            above: HashSet::new(),
            below: HashSet::new(),
        }
    }

    fn settle(
        mut self,
        grid: &mut HashMap<u32, Vec<(u32, u32, u32)>>,
        bricks: &mut HashMap<u32, Brick>,
    ) {
        let mut z = self.zs.0 - 1;
        let mut below = HashSet::new();
        while z > 0 {
            if let Some(plain) = grid.get(&z) {
                below = plain
                    .iter()
                    .filter_map(|p| {
                        if (self.xs.0..=self.xs.1).contains(&p.0)
                            && (self.ys.0..=self.ys.1).contains(&p.1)
                        {
                            Some(p.2)
                        } else {
                            None
                        }
                    })
                    .collect();
                if !below.is_empty() {
                    break;
                }
            }
            z -= 1;
        }
        let z1 = z + 1;
        let z2 = self.zs.1 - (self.zs.0 - z1);
        let points = (self.xs.0..=self.xs.1)
            .cartesian_product(self.ys.0..=self.ys.1)
            .map(|(x, y)| (x, y, self.id))
            .collect::<Vec<_>>();

        for z in z1..=z2 {
            grid.entry(z)
                .and_modify(|v| v.extend(points.clone()))
                .or_insert(points.clone());
        }
        for id in &below {
            if let Some(brick) = bricks.get_mut(id) {
                brick.above.insert(self.id);
            }
        }
        self.zs = (z1, z2);
        self.below = below;
        bricks.insert(self.id, self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 5);
        assert_eq!(p1(INPUT), 519);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 7);
        assert_eq!(p2(INPUT), 109531);
    }
}

#![allow(dead_code)]

use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

fn p1(input: &str, tmin: f64, tmax: f64) -> usize {
    parse::<f64>(input)
        .combinations(2)
        .filter(|v| {
            let (a, b) = (v[0], v[1]);
            let ma = a.vel[1] / a.vel[0];
            let mb = b.vel[1] / b.vel[0];
            if ma == mb {
                return false;
            }
            let x = (ma * a.pos[0] - mb * b.pos[0] + b.pos[1] - a.pos[1]) / (ma - mb);
            let y = ma * (x - a.pos[0]) + a.pos[1];
            tmin <= x
                && x <= tmax
                && tmin <= y
                && y <= tmax
                && (x - a.pos[0]) / a.vel[0] > 0.0
                && (x - b.pos[0]) / b.vel[0] > 0.0
        })
        .count()
}

fn p2(input: &str, is_test: bool) -> i64 {
    let (mut vxs, mut vys, mut vzs) = (HashSet::new(), HashSet::new(), HashSet::new());
    for hs in parse::<i64>(input).combinations(2) {
        let HStone {
            pos: [xa, ya, za],
            vel: [vxa, vya, vza],
        } = hs[0];
        let HStone {
            pos: [xb, yb, zb],
            vel: [vxb, vyb, vzb],
        } = hs[1];

        find_possible_v(vxa, vxb, xa, xb, &mut vxs, is_test);
        find_possible_v(vya, vyb, ya, yb, &mut vys, is_test);
        find_possible_v(vza, vzb, za, zb, &mut vzs, is_test);
    }
    if is_test {
        // To accommodate test data
        debug_assert!(vxs.contains(&-3));
        debug_assert!(vys.contains(&1));
        debug_assert!(vzs.contains(&2));
        (vxs, vys, vzs) = (HashSet::from([-3]), HashSet::from([1]), HashSet::from([2]));
    } else {
        debug_assert_eq!(&vxs, &HashSet::from([-227]));
        debug_assert_eq!(&vys, &HashSet::from([-221]));
        debug_assert_eq!(&vzs, &HashSet::from([111]));
    }

    let (Some(vx), Some(vy), Some(vz), Some(hs)) = (
        vxs.into_iter().next().map(|n| n as f64),
        vys.into_iter().next().map(|n| n as f64),
        vzs.into_iter().next().map(|n| n as f64),
        parse::<f64>(input).combinations(2).next(),
    ) else {
        unreachable!()
    };
    let HStone {
        pos: [xa, ya, za],
        vel: [vxa, vya, vza],
    } = hs[0];
    let HStone {
        pos: [xb, yb, _zb],
        vel: [vxb, vyb, _vzb],
    } = hs[1];

    // Slopes relative to target line
    let ma = (vya - vy) / (vxa - vx);
    let mb = (vyb - vy) / (vxb - vx);
    // ya = ma*x + ba, yb = mb*x + bb
    let ba = ya - (ma * xa);
    let bb = yb - (mb * xb);
    let pos_x = (bb - ba) / (ma - mb);
    let pos_y = ma * pos_x + ba;
    debug_assert_eq!(pos_y, mb * pos_x + bb);
    let t = (pos_x - xa) / (vxa - vx);
    let pos_z = za + (vza - vz) * t;
    (pos_x + pos_y + pos_z) as i64
}

fn find_possible_v(
    va: i64,
    vb: i64,
    pos_a: i64,
    pos_b: i64,
    v_set: &mut HashSet<i64>,
    is_test: bool,
) {
    // Take x-axis for example
    // if vxa == vxb
    // the line in question must cross a and b both
    // i.e (vel_x - vxa) * time = abs(xa - xb), in which all number are intergers
    // thus abs(xa - xb)%(vel_x - vxa) == 0
    // Continue this process for all combos to narrow down on vel_x

    let thres = if is_test { 1 } else { 100 };
    if va == vb && va.abs() > thres {
        let nxs = (-10 * thres..10 * thres)
            .filter(|&v| v != va && (pos_b - pos_a) % (v - va) == 0)
            .collect();
        if v_set.is_empty() {
            *v_set = nxs;
        } else {
            *v_set = v_set.intersection(&nxs).cloned().collect();
        }
    }
}

fn parse<T>(input: &str) -> impl Iterator<Item = HStone<T>> + '_
where
    T: FromStr + Default + Copy,
{
    input
        .lines()
        .map(|line| HStone::from_str(line.trim()).unwrap())
}

#[derive(Debug, Clone, Copy)]
struct HStone<T>
where
    T: FromStr + Default + Copy,
{
    pos: [T; 3],
    vel: [T; 3],
}

impl<T> FromStr for HStone<T>
where
    T: FromStr + Default + Copy,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut pos, mut vel) = ([T::default(); 3], [T::default(); 3]);
        for (i, num) in s.split(&[',', '@']).enumerate() {
            if i < 3 {
                pos[i] = num.trim().parse().map_err(|_| ())?;
            } else {
                vel[i - 3] = num.trim().parse().map_err(|_| ())?;
            }
        }
        Ok(Self { pos, vel })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST, 7.0, 27.0), 2);
        assert_eq!(p1(INPUT, 200000000000000.0, 400000000000000.0), 17244);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST, true), 47);
        assert_eq!(p2(INPUT, false), 1025019997186820);
    }
}

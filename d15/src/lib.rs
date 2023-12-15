#![allow(dead_code)]

use std::{collections::HashMap, str::FromStr};

fn p1(input: &str) -> u32 {
    input.trim().split(',').map(|s| hash(s.as_bytes())).sum()
}

fn p2(input: &str) -> u32 {
    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();
    for s in input.trim().split(',') {
        let lens = Lens::from_str(s).unwrap();
        let id = hash(&lens.label);
        match (boxes.get_mut(&id), lens.focal) {
            (Some(b), Some(_)) => {
                if let Some(idx) = b.iter().position(|item| item.label == lens.label) {
                    b[idx] = lens
                } else {
                    b.push(lens)
                }
            }
            (Some(b), None) => {
                if let Some(idx) = b.iter().position(|item| item.label == lens.label) {
                    b.remove(idx);
                }
            }
            (None, Some(_)) => {
                boxes.insert(id, vec![lens]);
            }
            (None, None) => (),
        }
    }
    boxes
        .into_iter()
        .flat_map(|(id, v)| {
            v.into_iter()
                .enumerate()
                .map(move |(idx, item)| (id + 1) * (idx as u32 + 1) * item.focal.unwrap_or(1))
        })
        .sum()
}

fn hash(s: &[u8]) -> u32 {
    s.iter().fold(0, |curr, &b| (curr + b as u32) * 17 % 256)
}

#[derive(Debug, Clone)]
struct Lens {
    label: Vec<u8>,
    focal: Option<u32>,
}

impl FromStr for Lens {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(&['=', '-']);
        let label = it.next().map(|s| s.as_bytes().to_vec()).ok_or(())?;
        // '=' => Some; '-' => None
        let focal = it.next().and_then(|s| s.parse().ok());
        Ok(Self { label, focal })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &'static str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 1320);
        assert_eq!(p1(INPUT), 514639);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(TEST), 145);
        assert_eq!(p2(INPUT), 279470);
    }
}

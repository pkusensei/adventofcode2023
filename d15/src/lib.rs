#![allow(dead_code)]

use std::collections::HashMap;

fn p1(input: &str) -> u32 {
    input.trim().split(',').map(hash).sum()
}

fn p2(input: &str) -> u32 {
    let mut boxes: HashMap<u32, Vec<Lens>> = HashMap::new();
    for s in input.trim().split(',') {
        let lens = Lens::from(s);
        let id = hash(lens.label);
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
                .map(move |(idx, item)| (id + 1) * (idx as u32 + 1) * item.focal.unwrap())
        })
        .sum()
}

fn hash(s: &str) -> u32 {
    s.bytes().fold(0, |curr, b| (curr + b as u32) * 17 % 256)
}

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    focal: Option<u32>,
}

// FromStr cannot take 'a
impl<'a> From<&'a str> for Lens<'a> {
    fn from(value: &'a str) -> Self {
        let mut it = value.split(&['=', '-']);
        let label = it.next().unwrap();
        // '=' => Some; '-' => None
        let focal = it.next().and_then(|s| s.parse().ok());
        Self { label, focal }
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

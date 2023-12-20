#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

fn p1(input: &'static str) -> usize {
    const COUNT: usize = 1000;
    let mut modules = setup(input);
    let start = modules.clone();
    let (mut low, mut high) = (0, 0);
    let mut i = 0;
    while i < COUNT {
        let (l, h) = run(&mut modules);
        i += 1;
        low += l;
        high += h;
        if start == modules {
            let loop_size = i;
            let loops = COUNT / loop_size;
            i = COUNT - COUNT % loop_size;
            low *= loops;
            high *= loops;
        }
    }
    low * high
}

fn p2(input: &'static str) -> usize {
    let modules = setup(input);

    // &bb -> rx
    // &bb needs all Pulse::High input to send P::Low
    // All nodes feeding into &bb needs to send P::High

    let bb = modules
        .iter()
        .find_map(|(name, m)| {
            if m.out.contains(&"rx") {
                Some(name)
            } else {
                None
            }
        })
        .unwrap();
    modules
        .iter()
        .filter_map(|(&name, m)| {
            if m.out.contains(bb) {
                // find nodes feeding into &bb
                Some(run2(&mut modules.clone(), name))
            } else {
                None
            }
        })
        .fold(1, utils::lcm)
}

fn setup(input: &'static str) -> HashMap<&str, Module> {
    let mut modules: HashMap<_, _> = parse(input).map(|m| (m.name, m)).collect();
    for module in parse(input) {
        for out in module.out {
            if let Some(v) = modules.get_mut(&out).and_then(|m| {
                if let ModType::Conj(ref mut ins) = m.type_ {
                    Some(ins)
                } else {
                    None
                }
            }) {
                v.insert(module.name, Pulse::Low);
            }
        }
    }
    modules
}

const SN: &str = "broadcaster";

fn run(modules: &mut HashMap<&'static str, Module>) -> (usize, usize) {
    let start = modules
        .get_mut(SN)
        .and_then(|m| m.send(Pulse::Low, ""))
        .unwrap();
    let mut low_count = 1; // button -> broadcaster
    let mut high_count = 0;
    let mut queue = VecDeque::from([(SN, start)]);
    while let Some((from, (pulse, names))) = queue.pop_front() {
        if let Pulse::Low = pulse {
            low_count += names.len()
        } else {
            high_count += names.len()
        }
        for name in names {
            if let Some(out) = modules.get_mut(name).and_then(|m| m.send(pulse, from)) {
                queue.push_back((name, out))
            }
        }
    }
    (low_count, high_count)
}

fn run2(modules: &mut HashMap<&'static str, Module>, target: &str) -> usize {
    let mut count = 0;
    'outer: loop {
        count += 1;
        let start = modules
            .get_mut(SN)
            .and_then(|m| m.send(Pulse::Low, ""))
            .unwrap();
        let mut queue = VecDeque::from([(SN, start)]);
        while let Some((from, (pulse, names))) = queue.pop_front() {
            if from == target && pulse == Pulse::High {
                break 'outer;
            }
            for name in names {
                if let Some(out) = modules.get_mut(name).and_then(|m| m.send(pulse, from)) {
                    queue.push_back((name, out));
                }
            }
        }
    }
    count
}

fn parse(input: &'static str) -> impl Iterator<Item = Module> {
    input.lines().map(|line| {
        let mut it = line.split("->");
        let n = it.next().unwrap().trim();
        let (type_, name) = match n.bytes().next() {
            Some(b'b') => (ModType::BCast, n),
            Some(b'%') => (ModType::Flip(false), n.split_at(1).1),
            Some(b'&') => (ModType::Conj(HashMap::new()), n.split_at(1).1),
            _ => unreachable!(),
        };
        let out = it.next().unwrap().split(',').map(|s| s.trim()).collect();
        Module { name, type_, out }
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModType {
    Flip(bool),
    Conj(HashMap<&'static str, Pulse>),
    BCast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    name: &'static str,
    type_: ModType,
    out: Vec<&'static str>,
}

impl Module {
    fn send(&mut self, pulse: Pulse, from: &'static str) -> Option<(Pulse, Vec<&'static str>)> {
        match (&mut self.type_, pulse) {
            (ModType::BCast, _) => Some((Pulse::Low, self.out.clone())),
            (ModType::Flip(_), Pulse::High) => None,
            (ModType::Flip(b), Pulse::Low) => {
                *b = !*b;
                if *b {
                    Some((Pulse::High, self.out.clone())) // Turns on
                } else {
                    Some((Pulse::Low, self.out.clone()))
                }
            }
            (ModType::Conj(ins), _) => {
                ins.insert(from, pulse);
                if ins.values().all(|p| *p == Pulse::High) {
                    Some((Pulse::Low, self.out.clone()))
                } else {
                    Some((Pulse::High, self.out.clone()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    const TEST2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test1() {
        assert_eq!(p1(TEST), 32000000);
        assert_eq!(p1(TEST2), 11687500);
        assert_eq!(p1(INPUT), 883726240);
    }

    #[test]
    fn test2() {
        assert_eq!(p2(INPUT), 211712400442661);
    }
}

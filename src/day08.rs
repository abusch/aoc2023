#![allow(dead_code)]
use std::collections::HashMap;

use aoc2023::Day;
use color_eyre::Result;
use once_cell::sync::Lazy;
use regex::Regex;

inventory::submit! {
    Day::new(8, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let instructions = lines[0];
    let graph = lines[2..].iter().map(|line| parse_node(line)).fold(
        Graph::new(),
        |mut graph, (node, left, right)| {
            graph.insert_node(node, left, right);
            graph
        },
    );
    let count = graph.run_instructions(instructions);

    Ok(format!("{count}"))
}

fn part2(input: &str) -> Result<String> {
    unimplemented!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Inst {
    L,
    R,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Label(String);

impl From<&str> for Label {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    left: Option<Label>,
    right: Option<Label>,
}

struct Graph {
    nodes: HashMap<Label, Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn insert_node(&mut self, node: Label, left: Label, right: Label) {
        self.nodes.insert(
            node,
            Node {
                left: Some(left),
                right: Some(right),
            },
        );
    }

    pub fn run_instructions(&self, instructions: &str) -> usize {
        let stop = "ZZZ".into();
        let mut instr = instructions.chars().cycle();
        let mut current = "AAA".into();
        let mut count = 0;
        loop {
            let node = self.nodes.get(&current).expect("Node not found!");
            let next = match instr.next() {
                Some('L') => node.left.clone(),
                Some('R') => node.right.clone(),
                _ => unreachable!(),
            };
            count += 1;
            match next {
                Some(next) if next == stop => break,
                Some(next) => {
                    current = next;
                }
                _ => unreachable!(),
            }
        }

        count
    }
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap());

fn parse_node(input: &str) -> (Label, Label, Label) {
    let captures = RE.captures(input).unwrap();
    let node = captures.get(1).unwrap().as_str().into();
    let left = captures.get(2).unwrap().as_str().into();
    let right = captures.get(3).unwrap().as_str().into();
    (node, left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let res = part1(
            r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        )
        .unwrap();

        assert_eq!(res, "2");
    }
}

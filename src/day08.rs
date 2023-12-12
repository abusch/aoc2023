#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    iter::Cycle,
    str::Chars,
};

use aoc2023::Day;
use color_eyre::Result;
use once_cell::sync::Lazy;
use regex::Regex;

inventory::submit! {
    Day::new(8, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let graph = parse_graph(input);
    let count = graph.run_instructions();

    Ok(format!("{count}"))
}

fn part2(input: &str) -> Result<String> {
    let graph = parse_graph(input);
    let count = graph.run_instructions_part2();

    Ok(format!("{count}"))
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Label([u8; 3]);

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Label")
            .field(&std::str::from_utf8(&self.0).unwrap())
            .finish()
    }
}

impl Label {
    pub fn is_start(&self) -> bool {
        &self.0 == b"AAA"
    }

    pub fn is_start2(&self) -> bool {
        self.0[2] == b'A'
    }

    pub fn is_end(&self) -> bool {
        &self.0 == b"ZZZ"
    }

    pub fn is_end2(&self) -> bool {
        self.0[2] == b'Z'
    }
}

impl From<&str> for Label {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().try_into().unwrap())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    left: Label,
    right: Label,
}

struct Graph {
    input: String,
    nodes: HashMap<Label, Node>,
}

impl Graph {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
            nodes: HashMap::new(),
        }
    }

    pub fn insert_node(&mut self, node: Label, left: Label, right: Label) {
        self.nodes.insert(node, Node { left, right });
    }

    pub fn follow_graph(&self, start_node: Label) -> GraphIter {
        GraphIter::new(self, start_node)
    }

    pub fn run_instructions(&self) -> usize {
        let start = "AAA".into();
        let end = "ZZZ".into();
        let pos = self
            .follow_graph(start)
            .position(|node| node == end)
            .expect("End node not found!");
        // pos is zero-based, so needs to add 1 for number of steps
        pos + 1
    }

    fn starting_nodes(&self) -> Vec<Label> {
        self.nodes
            .keys()
            .filter(|label| label.is_start2())
            .cloned()
            .collect()
    }

    pub fn run_instructions_part2(&self) -> u128 {
        self.starting_nodes()
            .into_iter()
            .map(|node| {
                let cycle = self.detect_cycle(node).unwrap();
                (cycle.0) as u128
            })
            .reduce(lcm)
            .unwrap()
    }

    pub fn detect_cycle(&self, start_node: Label) -> Option<(usize, Label)> {
        let input_len = self.input.len();

        let mut seen = HashSet::new();
        for (idx, node) in self
            .follow_graph(start_node)
            .enumerate()
            .filter(|(_idx, node)| node.is_end2())
        {
            let idx = idx + 1;
            let mod_idx = idx % input_len;
            if seen.contains(&(mod_idx, node)) || mod_idx == 0 {
                println!("{start_node}: {idx}, {mod_idx}, {node}");
                return Some((idx, node));
            } else {
                seen.insert((mod_idx, node));
            }
        }
        None
    }
}

struct GraphIter<'g> {
    graph: &'g Graph,
    input_iter: Cycle<Chars<'g>>,
    current_node: Label,
}

impl<'g> GraphIter<'g> {
    pub fn new(graph: &'g Graph, start_node: Label) -> Self {
        Self {
            graph,
            input_iter: graph.input.chars().cycle(),
            current_node: start_node,
        }
    }
}

impl<'g> Iterator for GraphIter<'g> {
    type Item = Label;

    fn next(&mut self) -> Option<Self::Item> {
        let inst = self.input_iter.next().expect("can't run out of input");
        let current = self
            .graph
            .nodes
            .get(&self.current_node)
            .expect("Node not found!");
        let next = match inst {
            'L' => current.left,
            'R' => current.right,
            _ => panic!("Invalid input!"),
        };
        self.current_node = next;
        Some(next)
    }
}

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(.{3}) = \((.{3}), (.{3})\)").unwrap());

fn parse_graph(input: &str) -> Graph {
    let lines = input.lines().collect::<Vec<_>>();
    let instructions = lines[0];
    lines[2..].iter().map(|line| parse_node(line)).fold(
        Graph::new(instructions),
        |mut graph, (node, left, right)| {
            graph.insert_node(node, left, right);
            graph
        },
    )
}

fn parse_node(input: &str) -> (Label, Label, Label) {
    let captures = RE.captures(input).unwrap();
    let node = captures.get(1).unwrap().as_str().into();
    let left = captures.get(2).unwrap().as_str().into();
    let right = captures.get(3).unwrap().as_str().into();
    (node, left, right)
}

fn lcm(a: u128, b: u128) -> u128 {
    (a * b) / gcd::euclid_u128(a, b)
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

    #[test]
    fn test_part2() {
        let res = part2(
            r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        )
        .unwrap();

        assert_eq!(res, "6");
    }
}

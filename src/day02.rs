use std::ops::Add;

use aoc2023::Day;
use color_eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u32},
    combinator::{map, value},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

inventory::submit! {
    Day::new(2, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let total: u32 = input
        .lines()
        .map(|line| {
            let (_, game) = parse_game(line).expect("Invalid line");
            game
        })
        .filter_map(|game| game.matches(12, 13, 14).then_some(game.0))
        .sum();

    Ok(format!("{total}"))
}

fn part2(input: &str) -> Result<String> {
    let total: u32 = input
        .lines()
        .map(|line| {
            let (_, game) = parse_game(line).expect("Invalid line");
            game
        })
        .map(|game| {
            let set = game.min_color_set();
            set.power()
        })
        .sum();

    Ok(format!("{total}"))
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ColorSet {
    r: u32,
    g: u32,
    b: u32,
}

impl ColorSet {
    pub fn max(&self, other: &Self) -> Self {
        Self {
            r: self.r.max(other.r),
            g: self.g.max(other.g),
            b: self.b.max(other.b),
        }
    }

    pub fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

impl Add<Grab> for ColorSet {
    type Output = Self;

    fn add(self, grab: Grab) -> Self::Output {
        let n = grab.0;
        match grab.1 {
            Color::Red => Self { r: n, ..self },
            Color::Green => Self { g: n, ..self },
            Color::Blue => Self { b: n, ..self },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Grab(u32, Color);

pub struct Round(Vec<Grab>);

impl Round {
    pub fn matches(&self, red: u32, green: u32, blue: u32) -> bool {
        self.0.iter().all(|grab| match grab.1 {
            Color::Red => grab.0 <= red,
            Color::Green => grab.0 <= green,
            Color::Blue => grab.0 <= blue,
        })
    }

    pub fn color_set(&self) -> ColorSet {
        self.0
            .iter()
            .fold(ColorSet::default(), |acc, grab| acc + *grab)
    }
}

pub struct Game(u32, Vec<Round>);

impl Game {
    pub fn matches(&self, red: u32, green: u32, blue: u32) -> bool {
        self.1.iter().all(|round| round.matches(red, green, blue))
    }

    pub fn min_color_set(&self) -> ColorSet {
        self.1.iter().fold(ColorSet::default(), |acc, round| {
            acc.max(&round.color_set())
        })
    }
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        value(Color::Red, tag("red")),
        value(Color::Green, tag("green")),
        value(Color::Blue, tag("blue")),
    ))(input)
}

fn parse_grab(input: &str) -> IResult<&str, Grab> {
    map(
        tuple((u32, space1, parse_color)),
        |(n, _, c): (u32, &str, Color)| Grab(n, c),
    )(input)
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    map(separated_list1(tag(", "), parse_grab), Round)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    map(
        tuple((
            tag("Game "),
            u32,
            tag(": "),
            separated_list1(tag("; "), parse_round),
        )),
        |(_, n, _, rounds)| Game(n, rounds),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let res = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        )
        .unwrap();
        assert_eq!(res, "2286")
    }
}

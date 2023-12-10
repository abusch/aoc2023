use aoc2023::Day;
use color_eyre::{eyre::Context, Result};
use regex::Regex;

inventory::submit! {
    Day::new(6, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let races = parse_races(input)?;

    let result: usize = races.into_iter().map(|r| r.num_record_beating()).product();

    Ok(format!("{result}"))
}

fn part2(input: &str) -> Result<String> {
    let race = parse_single_races(input)?;

    let result: usize = race.num_record_beating();

    Ok(format!("{result}"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    pub fn new(time: u64, dist: u64) -> Self {
        Self { time, dist }
    }

    pub fn num_record_beating(&self) -> usize {
        (0..self.time)
            .map(|t| {
                let remaining_t = self.time - t;
                let speed = t;

                speed * remaining_t
            })
            .filter(|d| *d > self.dist)
            .count()
    }
}

fn parse_races(input: &str) -> Result<Vec<Race>> {
    let lines = input.lines().collect::<Vec<_>>();
    let re = Regex::new(r"(\d+)").unwrap();
    let times = re
        .find_iter(lines[0])
        .map(|m| m.as_str().parse::<u64>().wrap_err("invalid number"))
        .collect::<Result<Vec<u64>>>()?;
    let distances = re
        .find_iter(lines[1])
        .map(|m| m.as_str().parse::<u64>().wrap_err("invalid number"))
        .collect::<Result<Vec<u64>>>()?;

    Ok(times
        .into_iter()
        .zip(distances)
        .map(|(t, d)| Race::new(t, d))
        .collect())
}

fn parse_single_races(input: &str) -> Result<Race> {
    let lines = input.lines().collect::<Vec<_>>();
    let re = Regex::new(r"(\d+)").unwrap();
    let time_str = re
        .find_iter(lines[0])
        .map(|m| m.as_str())
        .collect::<String>();
    let distance_str = re
        .find_iter(lines[1])
        .map(|m| m.as_str())
        .collect::<String>();

    Ok(Race::new(
        time_str.parse::<u64>()?,
        distance_str.parse::<u64>()?,
    ))
}

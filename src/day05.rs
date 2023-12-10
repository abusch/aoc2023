use std::collections::HashMap;

use aoc2023::Day;
use color_eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{space1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

inventory::submit! {
    Day::new(5, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let almanach = Almanach::parse(input)?;

    let min_location = almanach.part1();
    Ok(format!("{min_location}"))
}

fn part2(input: &str) -> Result<String> {
    let almanach = Almanach::parse(input)?;

    let min_location = almanach.part2();
    Ok(format!("{min_location}"))
}

struct Almanach {
    seeds: Vec<u64>,
    range_maps: HashMap<String, RangeMap>,
}

impl Almanach {
    pub fn parse(input: &str) -> Result<Self> {
        let lines = input.lines().collect::<Vec<_>>();
        let (_, seeds) = parse_seeds(lines[0]).unwrap();

        let mut maps = HashMap::new();
        let mut current_category: &str = "";
        let mut current_ranges = vec![];

        for line in &lines[2..] {
            if line.trim().is_empty() {
                maps.insert(
                    current_category.to_string(),
                    RangeMap::new(std::mem::take(&mut current_ranges)),
                );
                continue;
            }
            if let Some(category) = line.strip_suffix(" map:") {
                current_category = category;
                current_ranges = Vec::new();
                continue;
            }
            if let Ok((_, range)) = parse_range(line) {
                current_ranges.push(range);
            }
        }
        if !current_ranges.is_empty() {
            maps.insert(
                current_category.to_string(),
                RangeMap::new(std::mem::take(&mut current_ranges)),
            );
        }
        Ok(Self {
            seeds,
            range_maps: maps,
        })
    }

    pub fn part1(&self) -> u64 {
        self.seeds
            .iter()
            .copied()
            .map(|seed| self.seed_to_location(seed))
            .min()
            .unwrap()
    }

    pub fn part2(&self) -> u64 {
        self.seeds
            .chunks_exact(2)
            .map(|chunk| {
                let start = chunk[0];
                let len = chunk[1];
                (start..start + len)
                    .map(|seed| self.seed_to_location(seed))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.range_maps["seed-to-soil"].map(seed);
        let fertilizer = self.range_maps["soil-to-fertilizer"].map(soil);
        let water = self.range_maps["fertilizer-to-water"].map(fertilizer);
        let light = self.range_maps["water-to-light"].map(water);
        let temp = self.range_maps["light-to-temperature"].map(light);
        let humidity = self.range_maps["temperature-to-humidity"].map(temp);

        self.range_maps["humidity-to-location"].map(humidity)
    }
}

struct RangeMap(Vec<Range>);

impl RangeMap {
    pub fn new(mut ranges: Vec<Range>) -> Self {
        ranges.sort();
        Self(ranges)
    }

    pub fn map(&self, v: u64) -> u64 {
        let idx = self
            .0
            .binary_search_by_key(&v, |r| r.src_start)
            .unwrap_or_else(|x| x.saturating_sub(1));
        let range = self.0[idx];
        range.map(v).unwrap_or(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    src_start: u64,
    dst_start: u64,
    length: u64,
}

impl Range {
    pub fn map(&self, v: u64) -> Option<u64> {
        if v >= self.src_start && v < self.src_start + self.length {
            Some(self.dst_start + (v - self.src_start))
        } else {
            None
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.src_start.cmp(&other.src_start)
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    map(
        tuple((tag("seeds: "), separated_list1(space1, u64))),
        |(_, seeds)| seeds,
    )(input)
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    map(
        separated_pair(u64, tag(" "), separated_pair(u64, tag(" "), u64)),
        |(dst_start, (src_start, length))| Range {
            src_start,
            dst_start,
            length,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let (_, range) = parse_range("60 56 37").unwrap();
        assert_eq!(range.map(20), None);
        assert_eq!(range.map(56), Some(60));
        assert_eq!(range.map(57), Some(61));
        assert_eq!(range.map(92), Some(96));
        assert_eq!(range.map(93), None);
    }

    #[test]
    fn test_part1_part2() {
        let a = Almanach::parse(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        )
        .unwrap();

        assert_eq!(a.part1(), 35);
        assert_eq!(a.part2(), 46);
    }
}

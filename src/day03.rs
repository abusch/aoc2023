use color_eyre::Result;
use itertools::Itertools;
use regex::{self, Regex};

pub struct Day;

impl aoc2023::Day for Day {
    fn part1(&self, input: &str) -> Result<String> {
        let schematics = Schematics::new(input);

        let total: u32 = schematics.part_numbers_sum();

        Ok(format!("{total}"))
    }

    fn part2(&self, input: &str) -> Result<String> {
        let schematics = Schematics::new(input);

        let total: u32 = schematics.gear_ratios().into_iter().sum();

        Ok(format!("{total}"))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NumSpan {
    num: u32,
    x_start: usize,
    x_end: usize,
    y: usize,
}

pub struct Schematics<'s> {
    width: usize,
    height: usize,
    lines: Vec<&'s str>,
    num_spans: Vec<NumSpan>,
}

impl<'s> Schematics<'s> {
    pub fn new(input: &'s str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        let re = Regex::new(r"\d+").expect("Invalid regex");

        let num_spans = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                re.find_iter(line).map(move |m| {
                    let x_start = m.start();
                    let x_end = x_start + m.len() - 1;
                    let num = m.as_str().parse().expect("should be a number");
                    NumSpan {
                        num,
                        x_start,
                        x_end,
                        y,
                    }
                })
            })
            .collect::<Vec<_>>();

        Self {
            width,
            height,
            lines,
            num_spans,
        }
    }

    pub fn part_numbers(&self) -> Vec<u32> {
        self.num_spans
            .iter()
            .filter_map(|s| {
                self.is_neighbour_symbol(s.x_start, s.x_end, s.y)
                    .then_some(s.num)
            })
            .collect()
    }

    pub fn part_numbers_sum(&self) -> u32 {
        self.part_numbers().into_iter().sum()
    }

    pub fn gear_ratios(&self) -> Vec<u32> {
        let mut potential_gears = self
            .num_spans
            .iter()
            .filter_map(|n| {
                self.star_neighbour(n.x_start, n.x_end, n.y)
                    .map(|(x, y)| (n.num, (x, y)))
            })
            .collect::<Vec<_>>();
        potential_gears.sort_by_key(|v| v.1);
        let stars_grouped_by_pos = potential_gears.into_iter().group_by(|(_n, pos)| *pos);
        stars_grouped_by_pos
            .into_iter()
            .map(|(_pos, group)| group.map(|g| g.0).collect::<Vec<_>>())
            .filter_map(|nums| (nums.len() == 2).then_some(nums.into_iter().product()))
            .collect::<Vec<_>>()
    }

    fn is_neighbour_symbol(&self, x_start: usize, x_end: usize, y: usize) -> bool {
        self.span_neighbours_pos(x_start, x_end, y)
            .map(|(i, j)| self.at(i, j))
            .any(|c| !c.is_ascii_digit() && c != b'.')
    }

    fn star_neighbour(&self, x_start: usize, x_end: usize, y: usize) -> Option<(usize, usize)> {
        self.span_neighbours_pos(x_start, x_end, y)
            .find(|(i, j)| self.at(*i, *j) == b'*')
    }

    fn span_neighbours_pos(
        &self,
        x_start: usize,
        x_end: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let left_bound = if x_start > 0 { x_start - 1 } else { 0 };
        let right_bound = (x_end + 1).min(self.width - 1);
        let top_bound = if y > 0 { y - 1 } else { 0 };
        let bottom_bound = (y + 1).min(self.height - 1);

        (left_bound..=right_bound)
            .cartesian_product(top_bound..=bottom_bound)
            .filter(move |(i, j)| *i < x_start || *i > x_end || *j != y)
    }

    fn at(&self, x: usize, y: usize) -> u8 {
        self.lines[y].as_bytes()[x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = Schematics::new(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        let total = s.part_numbers_sum();
        assert_eq!(4361, total);
    }
}

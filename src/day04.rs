use std::collections::{HashSet, VecDeque};

use aoc2023::Day;
use color_eyre::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

inventory::submit! {
    Day::new(4, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let cards = input
        .lines()
        .map(|line| parse_card(line).expect("invalid line!").1)
        .collect::<Vec<_>>();

    let total = cards.into_iter().map(|c| c.value()).sum::<u32>();
    Ok(format!("{total}"))
}

fn part2(input: &str) -> Result<String> {
    let cards = input
        .lines()
        .map(|line| parse_card(line).expect("invalid line!").1)
        .collect::<Vec<_>>();

    let mut to_process = cards.iter().map(|c| c.card_num).collect::<VecDeque<_>>();

    let mut count = 0;
    while let Some(n) = to_process.pop_front() {
        count += 1;
        let c = &cards[n as usize - 1];
        let matches = c.matches() as u32;
        if matches != 0 {
            let copies_range = n + 1..=n + matches;
            to_process.extend(copies_range);
        }
    }

    Ok(format!("{count}"))
}

struct Card {
    card_num: u32,
    winning_nums: HashSet<u32>,
    nums: Vec<u32>,
}

impl Card {
    pub fn matches(&self) -> usize {
        self.nums
            .iter()
            .filter(|n| self.winning_nums.contains(*n))
            .count()
    }

    pub fn value(&self) -> u32 {
        let count = self.matches();
        if count == 0 {
            0
        } else {
            1 << (count - 1)
        }
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map(
        tuple((
            delimited(tuple((tag("Card"), space1)), u32, tuple((tag(":"), space1))),
            separated_list1(space1, u32),
            tuple((space1, tag("|"), space1)),
            separated_list1(space1, u32),
        )),
        |(n, winning, _, nums)| Card {
            card_num: n,
            winning_nums: winning.into_iter().collect(),
            nums,
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let total = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        )
        .unwrap();

        assert_eq!(total, "13");
    }

    #[test]
    fn test_part2() {
        let total = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        )
        .unwrap();

        assert_eq!(total, "30");
    }
}

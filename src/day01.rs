use aoc2023::Day;
use color_eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{map_res, value},
    IResult,
};

inventory::submit! {
    Day::new(1, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let total: u32 = input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>();
            let d1 = digits.first().copied().expect("Line has no digits!");
            let d2 = digits.last().copied().expect("Line has no digits!");
            d1 * 10 + d2
        })
        .sum();

    Ok(format!("{total}"))
}

fn part2(input: &str) -> Result<String> {
    let total: u32 = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let digits = (0..bytes.len())
                .filter_map(|i| parse_digit(&bytes[i..]).ok().map(|(_rest, digit)| digit))
                .collect::<Vec<_>>();
            let d1 = digits.first().copied().expect("Line has no digits!");
            let d2 = digits.last().copied().expect("Line has no digits!");
            d1 * 10 + d2
        })
        .sum();

    Ok(format!("{total}"))
}

fn parse_digit(input: &[u8]) -> IResult<&[u8], u32> {
    alt((
        value(1, tag(b"one")),
        value(2, tag(b"two")),
        value(3, tag(b"three")),
        value(4, tag(b"four")),
        value(5, tag(b"five")),
        value(6, tag(b"six")),
        value(7, tag(b"seven")),
        value(8, tag(b"eight")),
        value(9, tag(b"nine")),
        map_res(satisfy(|c| c.is_ascii_digit()), |c| {
            c.to_digit(10).ok_or("Char should be a digit")
        }),
    ))(input)
}

use eyre::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{map, value},
    IResult,
};

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day01.txt")?;
    let total: u64 = data
        .lines()
        .map(|line| {
            let digits = line
                .bytes()
                .filter_map(|c| c.is_ascii_digit().then_some(c - b'0'))
                .collect::<Vec<_>>();
            let d1 = digits.first().copied().unwrap() as u64;
            let d2 = digits.last().copied().unwrap() as u64;
            d1 * 10 + d2
        })
        .sum();

    println!("Part1: {total}");

    let total: u64 = data
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let digits = (0..bytes.len())
                .filter_map(|i| parse_digit(&bytes[i..]).ok().map(|(_rest, digit)| digit))
                .collect::<Vec<_>>();
            let d1 = digits.first().copied().unwrap() as u64;
            let d2 = digits.last().copied().unwrap() as u64;
            d1 * 10 + d2
        })
        .sum();

    println!("Part2: {total}");

    Ok(())
}

fn parse_digit(input: &[u8]) -> IResult<&[u8], u8> {
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
        map(satisfy(|c| c.is_ascii_digit()), |c| (c as u8 - b'0')),
    ))(input)
}

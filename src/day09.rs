use aoc2023::Day;
use color_eyre::Result;
use nom::{bytes::complete::tag, character::complete::i64, multi::separated_list1, IResult};

inventory::submit! {
    Day::new(9, part1, part2)
}

fn part1(input: &str) -> Result<String> {
    let sequences = input
        .lines()
        .map(|line| parse_history(line).expect("invalid input").1)
        .collect::<Vec<_>>();

    let total: i64 = sequences.into_iter().map(|seq| extrapolate(&seq)).sum();
    Ok(format!("{total}"))
}

fn part2(input: &str) -> Result<String> {
    let sequences = input
        .lines()
        .map(|line| parse_history(line).expect("invalid input").1)
        .collect::<Vec<_>>();

    let total: i64 = sequences.into_iter().map(|seq| extrapolate2(&seq)).sum();
    Ok(format!("{total}"))
}

fn extrapolate(seq: &[i64]) -> i64 {
    fn extrapolate_inner(seq: &[i64], sum: i64) -> i64 {
        if seq.iter().all(|v| *v == 0) {
            sum
        } else {
            let diffs = seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
            extrapolate_inner(&diffs, sum + seq.last().copied().unwrap())
        }
    }

    extrapolate_inner(seq, 0)
}

fn extrapolate2(seq: &[i64]) -> i64 {
    fn extrapolate_inner(seq: &[i64], sum: i64, factor: i64) -> i64 {
        if seq.iter().all(|v| *v == 0) {
            sum
        } else {
            let diffs = seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
            extrapolate_inner(
                &diffs,
                sum + factor * seq.first().copied().unwrap(),
                -factor,
            )
        }
    }

    extrapolate_inner(seq, 0, 1)
}
fn parse_history(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(" "), i64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_extrapolate2() {
        assert_eq!(extrapolate2(&[1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(extrapolate2(&[10, 13, 16, 21, 30, 45]), 5);
    }
}

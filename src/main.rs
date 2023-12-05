use aoc2023::AoC;
use color_eyre::Result;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() -> Result<()> {
    color_eyre::install()?;

    let aoc = AoC::new();

    if let Some(day) = std::env::args()
        .nth(1)
        .and_then(|n| n.parse::<usize>().ok())
    {
        aoc.run_day(day)?;
    } else {
        aoc.run_all_days()?;
    }

    Ok(())
}

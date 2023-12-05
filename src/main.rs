use aoc2023::AoC;
use color_eyre::Result;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() -> Result<()> {
    color_eyre::install()?;

    let aoc = AoC::new();

    aoc.run_day(4)?;

    Ok(())
}

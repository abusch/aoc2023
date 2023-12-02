use aoc2023::AoC;
use color_eyre::Result;

mod day01;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut aoc = AoC::default();
    aoc.register(1, day01::Day);

    aoc.run_day(1)?;

    Ok(())
}

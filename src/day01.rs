use eyre::Result;

pub fn run() -> Result<()> {
    let data = std::fs::read_to_string("inputs/day01.txt")?;

    Ok(())
}

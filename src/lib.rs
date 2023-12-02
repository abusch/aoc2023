use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use nu_ansi_term::{Color, Style};
use num_enum::IntoPrimitive;
use std::{collections::BTreeMap, fmt::Display};

#[derive(Copy, Clone, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum Part {
    P1 = 1,
    P2 = 2,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

pub trait Day {
    fn part1(&self, _input: &str) -> Result<String> {
        bail!("Not implemented yet!");
    }

    fn part2(&self, _input: &str) -> Result<String> {
        bail!("Not implemented yet!");
    }

    fn part(&self, part: Part, input: &str) -> Result<String> {
        match part {
            Part::P1 => self.part1(input),
            Part::P2 => self.part2(input),
        }
    }
}

pub struct Runner {
    n: usize,
    day: Box<dyn Day>,
}

impl Runner {
    pub fn new(n: usize, day: impl Day + 'static) -> Self {
        Self {
            n,
            day: Box::new(day),
        }
    }

    fn log_day(&self) {
        let s = format!(
            "Day {}",
            Style::default().bold().paint(format!("{:02}", self.n))
        );
        println!("{}", Color::Purple.underline().paint(s));
    }

    fn log_part_result(&self, part: Part, res: Result<String>) {
        let (color, msg) = match res {
            Ok(s) => (Color::Green, s),
            Err(e) => (Color::Red, e.to_string()),
        };
        println!(" → {}: {msg}", color.paint(format!("Part {part}")));
    }

    fn load_input(&self) -> Result<String> {
        std::fs::read_to_string(format!("inputs/day{:02}.txt", self.n))
            .wrap_err("Failed to load input file")
    }

    fn run_part(&self, part: Part, input: &str) {
        let res = self.day.part(part, input);
        self.log_part_result(part, res);
    }

    pub fn run(&self, part: impl Into<Option<Part>>) -> Result<()> {
        self.log_day();
        let input = self.load_input()?;
        match part.into() {
            Some(part) => self.run_part(part, &input),
            None => {
                self.run_part(Part::P1, &input);
                self.run_part(Part::P2, &input);
            }
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct AoC(BTreeMap<usize, Runner>);
impl AoC {
    pub fn register(&mut self, n: usize, day: impl Day + 'static) {
        self.0.insert(n, Runner::new(n, day));
    }

    pub fn run_day(&self, n: usize) -> Result<()> {
        if let Some(runner) = self.0.get(&n) {
            runner.run(None)?;
        } else {
            println!("Day {:02} not implemented yet!", n);
        }
        Ok(())
    }
}

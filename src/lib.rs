use color_eyre::{eyre::Context, Result};
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

inventory::collect!(Day);

type PartFn = fn(&str) -> Result<String>;

pub struct Day {
    /// Day number
    n: usize,
    /// Part 1
    part1: PartFn,
    /// Part 2
    part2: PartFn,
}

impl Day {
    pub const fn new(n: usize, part1: PartFn, part2: PartFn) -> Self {
        Self { n, part1, part2 }
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

    fn part(&self, part: Part, input: &str) -> Result<String> {
        match part {
            Part::P1 => (self.part1)(input),
            Part::P2 => (self.part2)(input),
        }
    }

    fn run_part(&self, part: Part, input: &str) {
        let res = self.part(part, input);
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
pub struct AoC(BTreeMap<usize, &'static Day>);
impl AoC {
    // pub fn register(&mut self, n: usize, day: impl Day + 'static) {
    //     self.0.insert(n, Runner::new(n, day));
    // }
    pub fn new() -> Self {
        let mut days = BTreeMap::default();
        for day in inventory::iter::<Day> {
            days.insert(day.n, day);
        }
        Self(days)
    }

    pub fn run_day(&self, n: usize) -> Result<()> {
        if let Some(day) = self.0.get(&n) {
            day.run(None)?;
        } else {
            println!("Day {:02} not implemented yet!", n);
        }
        Ok(())
    }

    pub fn run_all_days(&self) -> Result<()> {
        for day in self.0.values() {
            day.run(None)?;
        }

        Ok(())
    }
}

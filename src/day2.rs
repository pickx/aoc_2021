use anyhow::Result;
use parse_display::{Display, FromStr};

// #[derive(Display, FromStr, Debug, PartialEq, Eq, PartialOrd, Ord)]
// #[display("{0}")]
// struct Depth(u32);

#[derive(Default)]
struct Submarine {
    position: i64,
    depth: i64,
    aim: i64,
}

impl Submarine {}

#[derive(Display, FromStr, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[display("{} {0}", style = "lowercase")]
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn part1(input: &str) -> Result<i64> {
    let mut sub = Submarine::default();

    for instruction in input.lines().map(|line| line.parse::<Instruction>()) {
        let instruction = instruction?;
        match instruction {
            Instruction::Forward(d) => sub.position += i64::from(d),
            Instruction::Down(d) => sub.depth += i64::from(d),
            Instruction::Up(d) => sub.depth -= i64::from(d),
        }
    }
    Ok(sub.position * sub.depth)
}

pub fn part2(input: &str) -> Result<i64> {
    let mut sub = Submarine::default();

    for instruction in input.lines().map(|line| line.parse::<Instruction>()) {
        let instruction = instruction?;
        match instruction {
            Instruction::Forward(d) => {
                let d: i64 = d.into();
                sub.position += d;
                sub.depth += sub.aim * d;
            }
            Instruction::Down(d) => sub.aim += i64::from(d),
            Instruction::Up(d) => sub.aim -= i64::from(d),
        }
    }
    Ok(sub.position * sub.depth)
}

use anyhow::Result;
use itertools::Itertools;
use parse_display::{Display, FromStr};
// use ndarray::Array;
// use ndarray::Array2;

static INPUT: &str = include_str!("input/day8");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    // fn parse_from_chars(s: &str) -> Self {
    //     use Digit::*;
    //     let options = match s.len() {
    //         2 => vec![One],
    //         3 => vec![Seven],
    //         4 => vec![Four],
    //         5 => vec![Two, Three, Five],
    //         6 => vec![Zero, Six, Nine],
    //         7 => vec![Eight],
    //         _ => vec![],
    //     };

    // }

    fn required_segments(&self) -> Vec<Segment> {
        use Segment::*;
        match self {
            Digit::Zero => vec![A, B, C, E, F, G],
            Digit::One => vec![C, F],
            Digit::Two => vec![A, C, D, E, G],
            Digit::Three => vec![A, C, D, F, G],
            Digit::Four => vec![B, C, D, F],
            Digit::Five => vec![A, B, D, F, G],
            Digit::Six => vec![A, B, D, E, F, G],
            Digit::Seven => vec![A, C, F],
            Digit::Eight => vec![A, B, C, D, E, F, G],
            Digit::Nine => vec![A, B, C, D, F, G],
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

fn part1() {}

fn part2() {}

fn main() {
    part2();
}

use rayon::{iter::Either, prelude::*};
static INPUT: &str = include_str!("input/day10");

#[derive(Eq, PartialEq, Debug)]
enum Opener {
    Round,  //()
    Square, //[]
    Curly,  //{}
    Angle,  //<>
}

#[derive(Eq, PartialEq, Debug)]
enum Closer {
    Round,  //()
    Square, //[]
    Curly,  //{}
    Angle,  //<>
}

impl Opener {
    fn matching_closer(&self) -> Closer {
        match self {
            Opener::Round => Closer::Round,
            Opener::Square => Closer::Square,
            Opener::Curly => Closer::Curly,
            Opener::Angle => Closer::Angle,
        }
    }
}

impl Closer {
    fn matching_opener(&self) -> Opener {
        match self {
            Closer::Round => Opener::Round,
            Closer::Square => Opener::Square,
            Closer::Curly => Opener::Curly,
            Closer::Angle => Opener::Angle,
        }
    }

    fn corrupt_score(&self) -> u64 {
        match self {
            Closer::Round => 3,
            Closer::Square => 57,
            Closer::Curly => 1197,
            Closer::Angle => 25137,
        }
    }

    fn incomplete_score(&self) -> u64 {
        match self {
            Closer::Round => 1,
            Closer::Square => 2,
            Closer::Curly => 3,
            Closer::Angle => 4,
        }
    }
}

impl TryFrom<char> for Opener {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Opener::Round),
            '[' => Ok(Opener::Square),
            '{' => Ok(Opener::Curly),
            '<' => Ok(Opener::Angle),
            _ => Err("parsing failed"),
        }
    }
}

impl TryFrom<char> for Closer {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ')' => Ok(Closer::Round),
            ']' => Ok(Closer::Square),
            '}' => Ok(Closer::Curly),
            '>' => Ok(Closer::Angle),
            _ => Err("parsing failed"),
        }
    }
}

enum ParseResult {
    Corrupt(u64),
    Incomplete(Vec<Opener>),
}

fn parse_line(line: &str) -> ParseResult {
    let mut stack = Vec::new();

    for c in line.chars() {
        if let Ok(opener) = Opener::try_from(c) {
            stack.push(opener);
        } else if let Ok(closer) = Closer::try_from(c) {
            if stack.pop() != Some(closer.matching_opener()) {
                return ParseResult::Corrupt(closer.corrupt_score());
            }
        } else {
            panic!("found invalid character {c}");
        }
    }

    ParseResult::Incomplete(stack)
}

pub fn main() {
    let (corrupt_scores, mut incomplete_scores): (Vec<_>, Vec<_>) = INPUT
        .par_lines()
        .map(parse_line)
        .partition_map(|res| match res {
            ParseResult::Corrupt(score) => Either::Left(score),
            ParseResult::Incomplete(mut stack) => {
                let mut score = 0;
                while let Some(opener) = stack.pop() {
                    score *= 5;
                    score += opener.matching_closer().incomplete_score();
                }
                Either::Right(score)
            }
        });

    let syntax_error_score: u64 = corrupt_scores.par_iter().sum();

    let mid_idx = incomplete_scores.len() / 2;
    incomplete_scores.select_nth_unstable(mid_idx);
    let middle_score = incomplete_scores[mid_idx];

    println!("Score for Part 1: {syntax_error_score}");
    println!("Score for Part 2: {middle_score}");
}

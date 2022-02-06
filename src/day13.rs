use std::{collections::HashSet, ops::Mul};

use parse_display::{Display, FromStr};
use scan_fmt::scan_fmt;

static INPUT: &str = include_str!("input/day13");

#[derive(Copy, Clone, Debug, Display, FromStr)]
#[display("fold along {}={0}")]
#[display(style = "lowercase")]
enum Fold {
    X(usize),
    Y(usize),
}

fn fold_paper(points: &mut HashSet<(usize, usize)>, fold_line: Fold) {
    let to_move: Vec<_> = match fold_line {
        Fold::X(fold_line) => points
            .drain_filter(|(x, _)| x > &fold_line)
            .map(|(x, y)| (x - x.abs_diff(fold_line).mul(2), y))
            .collect(),
        Fold::Y(fold_line) => points
            .drain_filter(|(_, y)| y > &fold_line)
            .map(|(x, y)| (x, y - y.abs_diff(fold_line).mul(2)))
            .collect(),
    };

    points.extend(to_move);
}

fn draw_paper(points: &HashSet<(usize, usize)>) {
    let max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            let c = if points.contains(&(x, y)) { '#' } else { '.' };
            print!("{c}");
        }
        println!();
    }
}

pub fn main() {
    let (points_data, fold_data) = INPUT.split_once("\n\n").unwrap();

    let mut points: HashSet<(usize, usize)> = points_data
        .lines()
        .map(|line| scan_fmt!(line, "{},{}", usize, usize).unwrap())
        .collect();
    let folds: Vec<Fold> = fold_data
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    fold_paper(&mut points, folds[0]);

    println!("After first fold, {} points remain", points.len());

    for &fold_line in &folds[1..] {
        fold_paper(&mut points, fold_line);
    }

    draw_paper(&points);
}

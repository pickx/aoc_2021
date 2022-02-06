use std::cmp::{min, Reverse};

use itertools::Itertools;
use ndarray::{Array, Array2, ArrayBase, Dim, OwnedRepr};
use priority_queue::PriorityQueue;

static INPUT: &str = include_str!("input/day15");
static INPUT2: &str = "19999
19999
19111
19191
11191";
const ROWS: usize = 100;
const COLS: usize = 100;
const SCALE_FACTOR: usize = 5;

fn parse_num(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

fn neighbours_with_dim(
    (r, c): (usize, usize),
    (rows, cols): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::with_capacity(4);

    if r > 0 {
        neighbours.push((r - 1, c));
    }
    if r < rows - 1 {
        neighbours.push((r + 1, c));
    }
    if c > 0 {
        neighbours.push((r, c - 1));
    }
    if c < cols - 1 {
        neighbours.push((r, c + 1));
    }

    neighbours
}

// fn calc_risk_dynamic(risks: &Array2<u32>) -> u32 {
//     let (rows, cols) = risks.dim();
//     let mut dp = Array::zeros((rows + 1, cols + 1));
//
//     //set initial conditions
//     dp[(rows - 1, cols - 1)] = risks[(rows - 1, cols - 1)];
//     for r in (0..rows - 1).rev() {
//         dp[(r, cols - 1)] = dp[(r + 1, cols - 1)] + risks[(r, cols - 1)];
//     }
//     for c in (0..cols - 1).rev() {
//         dp[(rows - 1, c)] = dp[(rows - 1, c + 1)] + risks[(rows - 1, c)];
//     }
//
//     for r in (0..rows - 1).rev() {
//         for c in (0..cols - 1).rev() {
//             let risk = risks[(r, c)];
//             let from_below = risk.saturating_add(dp[(r + 1, c)]);
//             let from_right = risk.saturating_add(dp[(r, c + 1)]);
//             dp[(r, c)] = min(from_below, from_right);
//         }
//     }
//
//     //we don't actually visit the top left corner
//     dp[(0, 0)] - risks[(0, 0)]
// }

fn calc_risk(risk: &Array2<u32>) -> u32 {
    let (rows, cols) = risk.dim();
    let neighbours = |pos| neighbours_with_dim(pos, (rows, cols));

    // let mut parent: Array2<Option<(usize, usize)>> = Array::default((rows, cols));
    let mut distance: Array2<u32> =
        Array::from_shape_vec((rows, cols), vec![u32::MAX; rows * cols]).unwrap();

    let mut queue: PriorityQueue<(usize, usize), Reverse<u32>> = PriorityQueue::new();
    queue.push((0, 0), Reverse(0));
    distance[(0, 0)] = 0;

    // let neis = |(r, c): (usize, usize)| {
    //     let (r, c) = (r as isize, c as isize);
    //     let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)].iter();
    //     delta.map(|(dr, dc)| {

    //     };)
    // };
    while let Some((pos, Reverse(dist))) = queue.pop() {
        for nei in neighbours(pos) {
            let dist_of_nei = dist + risk[nei];
            if dist_of_nei < distance[nei] {
                distance[nei] = dist_of_nei;
                // parent[nei] = Some(pos);
                queue.push_decrease(nei, Reverse(dist_of_nei));

                // if nei == (rows - 1, cols - 1) {
                //     break;
                // }
            }
        }
    }

    distance[(rows - 1, cols - 1)]
}

fn fill_risk_matrix(
    base: &Array2<u32>,
    matrix: &mut Array2<u32>,
    inc: u32,
    (from_row, from_col): (usize, usize),
) {
    let (base_rows, base_cols) = base.dim();
    for r in 0..base_rows {
        for c in 0..base_cols {
            let base_val = base[(r, c)];
            let new_val = if base_val + inc <= 9 {
                base_val + inc
            } else {
                (base_val + inc) % 9
            };
            matrix[(r + from_row, c + from_col)] = new_val;
        }
    }
}

pub fn main() {
    let input_iter = INPUT.chars().filter(|c| !c.is_whitespace()).map(parse_num);
    let risk_base: Array2<u32> = Array::from_iter(input_iter)
        .into_shape((ROWS, COLS))
        .unwrap();

    let risk_expanded = {
        let mut expanded: Array2<u32> = Array::zeros((ROWS * SCALE_FACTOR, COLS * SCALE_FACTOR));
        for step in 0..SCALE_FACTOR {
            for height in 0..SCALE_FACTOR {
                let (from_row, from_col) = (height * ROWS, step * COLS);
                let inc: u32 = (height + step).try_into().unwrap();

                fill_risk_matrix(&risk_base, &mut expanded, inc, (from_row, from_col));
            }
        }
        expanded
    };

    println!("Risk in Part 1 is {}.", calc_risk(&risk_base));
    println!("Risk in Part 2 is {}.", calc_risk(&risk_expanded));
}

#[test]
fn feature() {
    let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)].iter();

    delta.for_each(|x| println!("{:?}", x))
}

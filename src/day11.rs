use itertools::Itertools;

// use ndarray::Array;
use crate::vec2d::Vec2D;

static INPUT: &str = include_str!("input/day11_bigger");
const ROWS: usize = 1000;
const COLS: usize = 1000;

fn surroundings((r, c): (usize, usize)) -> Vec<(usize, usize)> {
    let mut surroundings = Vec::new();

    if r > 0 {
        surroundings.push((r - 1, c));
        if c > 0 {
            surroundings.push((r - 1, c - 1));
        }
        if c < COLS - 1 {
            surroundings.push((r - 1, c + 1));
        }
    }
    if r < ROWS - 1 {
        surroundings.push((r + 1, c));
        if c > 0 {
            surroundings.push((r + 1, c - 1));
        }
        if c < COLS - 1 {
            surroundings.push((r + 1, c + 1));
        }
    }
    if c > 0 {
        surroundings.push((r, c - 1));
    }
    if c < COLS - 1 {
        surroundings.push((r, c + 1));
    }

    surroundings
}

pub fn main() {
    let octopi: Vec<_> = INPUT
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut octopi = Vec2D::new(COLS, octopi);

    let steps_for_part1 = 100;
    let mut total_flashes = 0;
    let mut first_step_of_sync = 0;

    for step in 0.. {
        let mut flashes_this_step = 0;
        let mut next_to_flash = Vec::new();

        for oct_pos in (0..ROWS).cartesian_product(0..COLS) {
            octopi[oct_pos] += 1;
            if octopi[oct_pos] == 10 {
                next_to_flash.push(oct_pos);
            }
        }

        while let Some(oct_pos) = next_to_flash.pop() {
            if octopi[oct_pos] == 0 {
                //already flashed.
                //note that it CANNOT be 0 if it has not flashed yet, because we already incremented it by stepping
                continue;
            }

            octopi[oct_pos] = 0;
            flashes_this_step += 1;

            for sur_pos in surroundings(oct_pos) {
                if octopi[sur_pos] == 0 {
                    continue;
                }

                octopi[sur_pos] += 1;
                if octopi[sur_pos] == 10 {
                    next_to_flash.push(sur_pos);
                }
            }
        }

        if step < steps_for_part1 {
            total_flashes += flashes_this_step;
        }

        if flashes_this_step == octopi.len() {
            first_step_of_sync = step + 1;
            break;
        }
    }

    println!("Total flashes for Part 1: {total_flashes}");
    println!("First step of sync for Part 2: {first_step_of_sync}");
}

use anyhow::Result;
use itertools::Itertools;
use ndarray::Array;
use ndarray::Array2;

static INPUT: &str = include_str!("input/day9");

fn part1() {
    let heightmap = INPUT
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).unwrap())
        .collect_vec();
    let cols = 100;
    let risk_level_total = heightmap
        .iter()
        .enumerate()
        .filter_map(|(idx, &height)| {
            let above = idx < cols || height < heightmap[idx - cols];
            let below = idx >= heightmap.len() - cols || height < heightmap[cols + idx];
            let left = idx % cols == 0 || height < heightmap[idx - 1];
            let right = (idx + 1) % cols == 0 || height < heightmap[idx + 1];

            if above && below && left && right {
                Some(height + 1)
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("Risk level is {risk_level_total}");
}

fn part2() {
    fn sink(arr: &mut Array2<u32>, (r, c): (usize, usize), basin_size: &mut u32) {
        if arr[(r, c)] == 9 {
            return;
        }

        arr[(r, c)] = 9; //sentinel value to avoid visiting here again

        *basin_size += 1;

        if r > 0 {
            sink(arr, (r - 1, c), basin_size);
        }
        if r < arr.dim().0 - 1 {
            sink(arr, (r + 1, c), basin_size);
        }
        if c > 0 {
            sink(arr, (r, c - 1), basin_size);
        }
        if c < arr.dim().1 - 1 {
            sink(arr, (r, c + 1), basin_size);
        }
    }

    let heightmap = INPUT
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).unwrap())
        .collect_vec();
    let mut arr = Array::from_shape_vec((100, 100), heightmap).unwrap();
    let mut basins = Vec::new();
    for r in 0..100 {
        for c in 0..100 {
            if arr[(r, c)] != 9 {
                let mut basin_size = 0;
                sink(&mut arr, (r, c), &mut basin_size);
                basins.push(basin_size);
            }
        }
    }
    basins.sort_unstable();

    let p = basins.iter().rev().take(3).product::<u32>();
    println!("Part 2 result: {p}");
}

fn main() {
    part2();
}

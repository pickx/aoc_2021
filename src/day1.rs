use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[display("{0}")]
struct Depth(u32);

pub fn part1(input: &str) -> u32 {
    let mut prev_depth = None;
    input
        .lines()
        .map(|s| s.parse::<Depth>().expect("parsing failed"))
        .fold(0_u32, |mut count, depth| {
            if let Some(prev_depth) = &prev_depth {
                if prev_depth < &depth {
                    count += 1
                }
            }

            prev_depth = Some(depth);

            count
        })
}

pub fn part2(input: &str) -> i32 {
    let depths: Vec<_> = input
        .lines()
        .map(|s| s.parse::<Depth>().expect("parsing failed"))
        .collect();
    let mut prev_sum = None;
    depths.windows(3).fold(0_i32, |mut count, window| {
        let sum = window.iter().map(|d| d.0).sum::<u32>();
        if let Some(prev_sum) = &prev_sum {
            if &sum > prev_sum {
                count += 1
            }
        }

        prev_sum = Some(sum);

        count
    })
}

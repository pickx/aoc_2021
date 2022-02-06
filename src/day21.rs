use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const START_POS_PLAYER_1: u32 = 5;
const START_POS_PLAYER_2: u32 = 9;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(starting_pos: u32) -> Player {
        Player {
            pos: starting_pos,
            score: 0,
        }
    }

    fn move_on_board(&mut self, spaces_to_move: u32) {
        // if self.pos + spaces < 10 { self.pos += spaces } else { self.pos += }

        self.pos = (self.pos + spaces_to_move) % 10;
        if self.pos == 0 {
            self.pos = 10;
        }

        self.score += self.pos;
    }

    fn has_won_part1(&self) -> bool {
        self.score >= 1000
    }
    fn has_won_part2(&self) -> bool {
        self.score >= 21
    }
}

fn determinstic_die(prev: u32) -> u32 {
    match prev {
        0..=99 => prev + 1,
        100 => 1,
        _ => panic!("bad input"),
    }
}

// fn dirac() -> u32 {

// }

fn roll(prev: u32, rng: impl Fn(u32) -> u32) -> u32 {
    rng(prev)
}

pub fn main() {
    let p1 = Player::new(START_POS_PLAYER_1);
    let p2 = Player::new(START_POS_PLAYER_2);

    let mut current_player = p1;
    let mut previous_player = p2;

    // let starting_dice_pos = 0;
    // let mut prev_dice_pos = starting_dice_pos;
    // let mut turns = 0;
    // loop {
    //     turns += 1;
    //     let spaces_to_move: u32 = (0..3)
    //         .map(|_| {
    //             let r = roll(prev_dice_pos, determinstic_die);
    //             prev_dice_pos = r;
    //             r
    //         })
    //         .sum();
    //     current_player.move_on_board(spaces_to_move);
    //     if current_player.has_won_part1() {
    //         break;
    //     }

    //     std::mem::swap(&mut current_player, &mut previous_player);
    // }

    // let losing_score = previous_player.score;
    // let dice_rolls = 3 * turns;
    // println!(
    //     "Part 1 complete after {} turns, losing score is {}, solution is {}",
    //     turns,
    //     losing_score,
    //     losing_score * dice_rolls
    // );

    let rolls_and_counts = [(3, 1), (4, 3), (6, 7), (8, 3), (5, 6), (7, 6), (9, 1)];
    let player1_wins = {
        let mut dimensions = HashMap::new();
        dimensions.insert(Player::new(START_POS_PLAYER_1), 1);
        let mut won = 0_u64;
        loop {
            let mut next_dimension = HashMap::new();
            for (p, p_count) in dimensions {
                for (roll, count) in rolls_and_counts.iter().copied() {
                    let mut p = p.clone();
                    p.move_on_board(roll);
                    if p.has_won_part2() {
                        won += p_count + count;
                    }
                    *next_dimension.entry(p).or_insert(p_count) += count;
                }
            }

            if won > 0 {
                break won;
            }

            dimensions = next_dimension;
        }
    };

    dbg!(player1_wins);
}

#[test]
fn moving() {
    let mut p = Player::new(4);
    p.move_on_board(91 + 92 + 93);
    p.move_on_board(3);
    dbg!(p);
}

#[test]
fn feature() {
    let options = {
        let mut options = Vec::new();
        for r1 in 1..=3 {
            for r2 in 1..=3 {
                for r3 in 1..=3 {
                    options.push((r1, r2, r3))
                }
            }
        }
        options
    };
    options
        .iter()
        .for_each(|(r1, r2, r3)| println!("({r1},{r2},{r3})"));

    for (sum, count) in options.iter().map(|(r1, r2, r3)| r1 + r2 + r3).counts() {
        println!("[{sum}, {count}]");
    }
}

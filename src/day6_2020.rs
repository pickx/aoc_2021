use std::collections::HashSet;

static INPUT: &str = include_str!("input_2020/day6");

pub fn main() {
    let mut any_said_yes: HashSet<char> = HashSet::new();
    let mut all_said_yes: HashSet<char> = HashSet::new();

    let mut any_total = 0;
    let mut all_total = 0;

    for group in INPUT.split("\n\n") {
        let mut new_group_started = true;

        for line in group.lines() {
            any_said_yes.extend(line.chars());

            if new_group_started {
                all_said_yes.extend(line.chars());
                new_group_started = false;
            } else {
                let cur_answers: HashSet<char> = line.chars().collect();
                all_said_yes.retain(|c| cur_answers.contains(c));
            }
        }

        any_total += any_said_yes.len();
        all_total += all_said_yes.len();

        any_said_yes.clear();
        all_said_yes.clear();
    }

    println!("Sum of counts in Part 1 {any_total}");
    println!("Sum of counts in Part 2 {all_total}");
}

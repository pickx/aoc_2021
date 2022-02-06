use std::ops::Add;

use anyhow::{Context, Result};
use itertools::Itertools;
use scan_fmt::scan_fmt;
use slab_tree::Tree;

static INPUT: &str = include_str!("input/day18");

#[derive(Debug, Clone)]
enum NodeKind {
    Leaf(u32),
    Branch(usize, usize),
}

#[derive(Debug)]
enum NumberKind {
    Pair(Number, Number),
    Regular(u32),
}

#[derive(Debug)]
struct Number {
    depth: usize,
    kind: Box<NumberKind>,
}

#[derive(Debug)]
struct NumberBase {}

impl Number {
    fn from_str(input: &str) -> Number {
        let mut stack = Vec::new();
        let mut depth = 0;
        for c in input.trim().chars() {
            match c {
                '[' => {
                    depth += 1;
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let n = c.to_digit(10).unwrap();
                    let regular = NumberKind::Regular(n);
                    let number = Number {
                        depth,
                        kind: Box::new(regular),
                    };
                    stack.push(number);
                }

                ']' => {
                    depth -= 1;

                    //note the pop ordering
                    let r = stack.pop().unwrap();
                    let l = stack.pop().unwrap();

                    let pair = NumberKind::Pair(l, r);
                    let number = Number {
                        depth,
                        kind: Box::new(pair),
                    };
                    stack.push(number);
                }

                ',' => (),

                _ => panic!("found unexpected character {c}"),
            }
        }

        assert_eq!(stack.len(), 1);
        let root = stack.pop().unwrap();
        let mut tree: Tree<Number> = Tree::new();
        tree.set_root(root);
        let root = tree.root_mut().unwrap();
        root.

        todo!()
    }

    fn reduce(&mut self) {
        if self.depth >= 5 {}
    }

    fn increase_depth(&mut self) {
        self.depth += 1;
        if let NumberKind::Pair(l, r) = self.kind.as_mut() {
            l.increase_depth();
            r.increase_depth();
        }
    }

    fn decrease_depth(&mut self) {
        self.depth -= 1;
        if let NumberKind::Pair(l, r) = self.kind.as_mut() {
            l.decrease_depth();
            r.decrease_depth();
        }
    }
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(mut self, mut rhs: Number) -> Self::Output {
        self.increase_depth();
        rhs.increase_depth();
        let pair = NumberKind::Pair(self, rhs);
        Number {
            depth: 0,
            kind: Box::new(pair),
        }
    }
}

pub fn main() {}

#[test]
fn parsing() {
    let input = "[[[[0,9],[1,4]],[[3,4],3]],[1,[[7,7],[3,5]]]]";
    dbg!(Number::from_str(input));
}

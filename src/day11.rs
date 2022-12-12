use std::collections::VecDeque;

use itertools::Itertools;
use regex::{Captures, Regex};

#[derive(Clone, Debug)]
enum Op {
    ADD(u64),
    MUL(u64),
    POW,
}

impl Op {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Op::ADD(x) => value + x,
            Op::MUL(x) => value * x,
            Op::POW => value * value,
        }
    }
}

#[derive(Clone, Debug)]
struct Test {
    divide_by: u64,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn check(&self, value: u64) -> usize {
        if value % self.divide_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test: Test,
}

impl Monkey {
    fn from_regex_capture(capture: &Captures) -> Self {
        Self {
            items: capture["items"]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect(),
            op: match (&capture["op"], &capture["val"]) {
                ("*", "old") => Op::POW,
                ("*", s) => Op::MUL(s.parse().unwrap()),
                ("+", s) => Op::ADD(s.parse().unwrap()),
                _ => panic!(),
            },
            test: Test {
                divide_by: capture["test"].parse().unwrap(),
                if_true: capture["true"].parse().unwrap(),
                if_false: capture["false"].parse().unwrap(),
            },
        }
    }
}

const MONKEY_REGEX_STRING: &'static str = r"Monkey \d:
\s+Starting items: (?P<items>\d+(?:, \d+)*)
\s+Operation: new = old (?P<op>\+|\*) (?P<val>\d+|old)
\s+Test: divisible by (?P<test>\d+)
\s+If true: throw to monkey (?P<true>\d+)
\s+If false: throw to monkey (?P<false>\d+)";

fn monkeys_from_input(input: &str) -> Vec<Monkey> {
    Regex::new(MONKEY_REGEX_STRING)
        .unwrap()
        .captures_iter(input)
        .map(|c| Monkey::from_regex_capture(&c))
        .collect_vec()
}

fn simulate20(mut monkeys: Vec<Monkey>) -> Vec<u64> {
    let mut inspection_count = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                inspection_count[i] += 1;
                let item = monkeys[i].items.pop_front().unwrap();
                let new_item = monkeys[i].op.apply(item) / 3;
                let j = monkeys[i].test.check(new_item);
                monkeys[j].items.push_back(new_item);
            }
        }
    }
    inspection_count
}

fn simulate10000(mut monkeys: Vec<Monkey>) -> Vec<u64> {
    let mut inspection_count = vec![0; monkeys.len()];
    let worry_dampener = monkeys
        .iter()
        .map(|m| m.test.divide_by)
        .reduce(|acc, x| acc * x)
        .unwrap();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                inspection_count[i] += 1;
                let item = monkeys[i].items.pop_front().unwrap();
                let new_item = monkeys[i].op.apply(item) % worry_dampener;
                let j = monkeys[i].test.check(new_item);
                monkeys[j].items.push_back(new_item);
            }
        }
    }
    inspection_count
}

fn find_top_2(vec: Vec<u64>) -> (u64, u64) {
    let mut max1 = 0;
    let mut max2 = 0;
    for x in vec {
        if x >= max1 {
            max2 = max1;
            max1 = x;
        } else if x > max2 {
            max2 = x;
        }
    }
    (max1, max2)
}

pub fn solve(input: String) -> (String, String) {
    let monkeys = monkeys_from_input(&input);

    let (max1, max2) = find_top_2(simulate20(monkeys.clone()));
    let result1 = max1 * max2;

    let (max1, max2) = find_top_2(simulate10000(monkeys));
    let result2 = max1 * max2;

    (
        format!("Monkey business: {result1}"),
        format!("Crazy monkey business: {result2}"),
    )
}

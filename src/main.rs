#![allow(non_snake_case)]

use std::fs;

mod day1;

static SOLUTIONS: &[fn() -> ()] = &[day1::solve];

fn main() {
    for solution in SOLUTIONS {
        solution();
    }
}

fn read_input(day: i8, test: bool) -> String {
    let file_path = format!("input/day{}{}.txt", day, if test { "_test" } else { "" });
    fs::read_to_string(file_path).unwrap()
}

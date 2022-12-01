#![allow(non_snake_case)]

use clap::Parser;
use std::fs;

mod day1;

static SOLUTIONS: &[fn(String) -> ()] = &[day1::solve];

#[derive(Parser, Debug)]
struct CLI {
    #[arg(short, long)]
    day: Option<usize>,
    #[arg(short, long)]
    test: bool,
}

fn main() {
    let args = CLI::parse();
    if let Some(day) = args.day {
        if let Some(solution) = SOLUTIONS.get(day - 1) {
            solution(read_input(day, args.test));
        } else {
            println!("There is no solution for day {}", day);
        }
    } else {
        for (index, solution) in SOLUTIONS.iter().enumerate() {
            solution(read_input(index + 1, args.test));
        }
    }
}

fn read_input(day: usize, test: bool) -> String {
    let file_path = format!("input/day{}{}.txt", day, if test { "_test" } else { "" });
    fs::read_to_string(file_path).unwrap()
}

#![allow(non_snake_case)]

use clap::Parser;
use std::fs;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

static SOLUTIONS: &[fn(String) -> (String, String)] = &[
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
    day7::solve,
    day8::solve,
    day9::solve,
    day10::solve,
    day11::solve,
    day12::solve,
    day13::solve,
];

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
            print_solution_for_day(solution(read_input(day, args.test)), day)
        } else {
            println!("!!! There is no solution for day {:02} !!!", day);
        }
    } else {
        for (index, solution) in SOLUTIONS.iter().enumerate() {
            print_solution_for_day(solution(read_input(index + 1, args.test)), index + 1);
        }
    }
}

fn print_solution_for_day(solution: (String, String), day: usize) {
    println!(
        "========= day {:02} =========\n\n{}\n{}\n\n==========================",
        day, solution.0, solution.1
    );
}

fn read_input(day: usize, test: bool) -> String {
    let file_path = format!("input/day{}{}.txt", day, if test { "_test" } else { "" });
    fs::read_to_string(file_path).unwrap()
}

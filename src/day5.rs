use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

type Stack = Vec<char>;

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction_regex: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let groups = instruction_regex.captures(s).unwrap();
        Ok(Self {
            count: groups.get(1).unwrap().as_str().parse().unwrap(),
            from: groups.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            to: groups.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        })
    }
}

fn process_stack_input(stack_input: &str) -> Vec<Stack> {
    let mut stack_lines = stack_input.lines().rev();
    let stack_count = stack_lines.next().unwrap().replace(" ", "").len();
    let stack_regex = Regex::new((0..stack_count).map(|_| r"(...)").join(" ").as_str()).unwrap();

    let mut stacks: Vec<Stack> = (0..stack_count).map(|_| Vec::new()).collect();

    for line in stack_lines {
        let groups = stack_regex.captures(line).unwrap();
        for i in 0..stack_count {
            let crate_char = groups.get(i + 1).unwrap().as_str().chars().nth(1).unwrap();
            if crate_char.is_alphabetic() {
                stacks[i].push(crate_char);
            }
        }
    }
    stacks
}

fn process_input(input: String) -> (Vec<Stack>, Vec<Instruction>) {
    let (stack_input, instruction_input) = input.split_once("\n\n").unwrap();

    let stacks = process_stack_input(stack_input);
    let instructions = instruction_input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();

    (stacks, instructions)
}

fn move_stacks_crane_9000(mut stacks: Vec<Stack>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let c = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(c)
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn move_stacks_crane_9001(mut stacks: Vec<Stack>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        let stack_size = stacks[instruction.from].len();
        let mut moved_crates = stacks[instruction.from].split_off(stack_size - instruction.count);
        stacks[instruction.to].append(&mut moved_crates);
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

pub fn solve(input: String) -> (String, String) {
    let (stacks, instructions) = process_input(input);

    let top_crates_9000 = move_stacks_crane_9000(stacks.clone(), &instructions);
    let top_crates_9001 = move_stacks_crane_9001(stacks.clone(), &instructions);

    (
        format!("Top crates (9000): {top_crates_9000}"),
        format!("Top crates (9001): {top_crates_9001}"),
    )
}

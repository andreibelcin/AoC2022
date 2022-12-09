use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

struct Instruction {
    direction: Direction,
    step: u32,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_str, step_str) = s.split_once(" ").unwrap();
        let direction = match direction_str {
            "R" => Direction::RIGHT,
            "L" => Direction::LEFT,
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            _ => panic!("Unknown instruction: {s}"),
        };

        Ok(Self {
            direction,
            step: step_str.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Rope {
    segments: Vec<(i32, i32)>,
    len: usize,
}

impl Rope {
    fn new(len: usize) -> Rope {
        let segments = vec![(0, 0); len];
        Rope { segments, len }
    }

    fn follow_instruction(&mut self, instruction: &Instruction) -> Vec<(i32, i32)> {
        let mut tail_tip_positions = Vec::new();

        let head_x_move = match instruction.direction {
            Direction::RIGHT => 1,
            Direction::LEFT => -1,
            _ => 0,
        };
        let head_y_move = match instruction.direction {
            Direction::UP => 1,
            Direction::DOWN => -1,
            _ => 0,
        };

        for _ in 0..instruction.step {
            let mut x_move = head_x_move;
            let mut y_move = head_y_move;

            for i in 0..(self.len - 1) {
                self.segments[i].0 += x_move;
                self.segments[i].1 += y_move;

                if self.segments[i].0.abs_diff(self.segments[i + 1].0) > 1
                    || self.segments[i].1.abs_diff(self.segments[i + 1].1) > 1
                {
                    x_move = (self.segments[i].0 - self.segments[i + 1].0).signum();
                    y_move = (self.segments[i].1 - self.segments[i + 1].1).signum();
                } else {
                    x_move = 0;
                    y_move = 0;
                }
            }

            self.segments[self.len - 1].0 += x_move;
            self.segments[self.len - 1].1 += y_move;
            tail_tip_positions.push(self.segments[self.len - 1]);
        }
        tail_tip_positions
    }
}

fn get_tail_positions_count(mut rope: Rope, instructions: &Vec<Instruction>) -> usize {
    let mut tail_positions = HashSet::new();
    tail_positions.insert(*rope.segments.last().unwrap());
    for instruction in instructions {
        tail_positions.extend(rope.follow_instruction(instruction));
    }
    println!("{rope:?}");
    tail_positions.len()
}

pub fn solve(input: String) -> (String, String) {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect_vec();

    let result1 = get_tail_positions_count(Rope::new(2), &instructions);
    let result2 = get_tail_positions_count(Rope::new(10), &instructions);

    (
        format!("Number of locations visited by tail: {result1}"),
        format!("Number of locations visited by long tail: {result2}"),
    )
}

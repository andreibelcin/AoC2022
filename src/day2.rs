use std::{cmp::Ordering, ops::Add, str::FromStr};

#[derive(PartialEq)]
enum Move {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Move::ROCK, Move::ROCK) => Ordering::Equal,
            (Move::ROCK, Move::PAPER) => Ordering::Less,
            (Move::ROCK, Move::SCISSORS) => Ordering::Greater,
            (Move::PAPER, Move::ROCK) => Ordering::Greater,
            (Move::PAPER, Move::PAPER) => Ordering::Equal,
            (Move::PAPER, Move::SCISSORS) => Ordering::Less,
            (Move::SCISSORS, Move::ROCK) => Ordering::Less,
            (Move::SCISSORS, Move::PAPER) => Ordering::Greater,
            (Move::SCISSORS, Move::SCISSORS) => Ordering::Equal,
        })
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::ROCK),
            "B" | "Y" => Ok(Self::PAPER),
            "C" | "Z" => Ok(Self::SCISSORS),
            _ => Err(()),
        }
    }
}

impl Add<Strategy> for Move {
    type Output = Move;

    fn add(self, rhs: Strategy) -> Self::Output {
        match (self, rhs) {
            (Move::ROCK, Strategy::WIN) => Move::PAPER,
            (Move::ROCK, Strategy::DRAW) => Move::ROCK,
            (Move::ROCK, Strategy::LOSE) => Move::SCISSORS,
            (Move::PAPER, Strategy::WIN) => Move::SCISSORS,
            (Move::PAPER, Strategy::DRAW) => Move::PAPER,
            (Move::PAPER, Strategy::LOSE) => Move::ROCK,
            (Move::SCISSORS, Strategy::WIN) => Move::ROCK,
            (Move::SCISSORS, Strategy::DRAW) => Move::SCISSORS,
            (Move::SCISSORS, Strategy::LOSE) => Move::PAPER,
        }
    }
}

enum Strategy {
    WIN = 6,
    DRAW = 3,
    LOSE = 0,
}

impl From<(Move, Move)> for Strategy {
    fn from(m: (Move, Move)) -> Self {
        if m.0 < m.1 {
            Self::WIN
        } else if m.0 > m.1 {
            Self::LOSE
        } else {
            Self::DRAW
        }
    }
}

impl FromStr for Strategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::LOSE),
            "Y" => Ok(Self::DRAW),
            "Z" => Ok(Self::WIN),
            _ => Err(()),
        }
    }
}

pub fn solve(input: String) -> (String, String) {
    let score_1: i32 = input
        .lines()
        .map(|l| {
            l.split_once(" ")
                .map(|p| (Move::from_str(p.0).unwrap(), Move::from_str(p.1).unwrap()))
                .unwrap()
        })
        .map(|r| (r.1 as i32) + (Strategy::from(r) as i32))
        .sum();

    let score_2: i32 = input
        .lines()
        .map(|l| {
            l.split_once(" ")
                .map(|p| {
                    (
                        Move::from_str(p.0).unwrap(),
                        Strategy::from_str(p.1).unwrap(),
                    )
                })
                .unwrap()
        })
        .map(|p| (p.1 as i32) + ((p.0 + p.1) as i32))
        .sum();

    (
        format!("Strategy 1 score: {score_1}"),
        format!("Strategy 2 score: {score_2}"),
    )
}

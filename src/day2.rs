use std::str::FromStr;

#[derive(Clone, Copy)]
enum Move {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl Move {
    fn against(&self, other: Move) -> Strategy {
        match (self, other) {
            (Move::ROCK, Move::ROCK) => Strategy::DRAW,
            (Move::ROCK, Move::PAPER) => Strategy::LOSE,
            (Move::ROCK, Move::SCISSORS) => Strategy::WIN,
            (Move::PAPER, Move::ROCK) => Strategy::WIN,
            (Move::PAPER, Move::PAPER) => Strategy::DRAW,
            (Move::PAPER, Move::SCISSORS) => Strategy::LOSE,
            (Move::SCISSORS, Move::ROCK) => Strategy::LOSE,
            (Move::SCISSORS, Move::PAPER) => Strategy::WIN,
            (Move::SCISSORS, Move::SCISSORS) => Strategy::DRAW,
        }
    }

    fn counter_move(&self, strategy: Strategy) -> Move {
        match (self, strategy) {
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

#[derive(Clone, Copy)]
enum Strategy {
    WIN = 6,
    DRAW = 3,
    LOSE = 0,
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
    let input: Vec<(_, _)> = input.lines().map(|l| l.split_once(" ").unwrap()).collect();

    let score_1: i32 = input
        .iter()
        .map(|p| (p.0.parse::<Move>().unwrap(), p.1.parse::<Move>().unwrap()))
        .map(|r| (r.1 as i32) + (r.1.against(r.0) as i32))
        .sum();

    let score_2: i32 = input
        .iter()
        .map(|p| {
            (
                p.0.parse::<Move>().unwrap(),
                p.1.parse::<Strategy>().unwrap(),
            )
        })
        .map(|p| (p.1 as i32) + (p.0.counter_move(p.1) as i32))
        .sum();

    (
        format!("Strategy 1 score: {score_1}"),
        format!("Strategy 2 score: {score_2}"),
    )
}

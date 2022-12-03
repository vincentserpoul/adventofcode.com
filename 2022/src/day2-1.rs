#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    clippy::cargo
)]

use eyre::Result;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let score = input
        .lines()
        .map(|l| l.parse::<Match>().unwrap().to_number())
        .sum::<u32>();

    println!("{}", score);

    Ok(())
}

use thiserror::Error;

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

use std::str::FromStr;

#[derive(Error, Debug)]
pub enum ShapeError {
    #[error("`{0}` is not a shape")]
    InvalidShape(String),
}

impl FromStr for Shape {
    type Err = ShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(ShapeError::InvalidShape(s.to_string())),
        }
    }
}

enum MatchResult {
    Lose,
    Draw,
    Win,
}

impl MatchResult {
    fn to_number(&self) -> u32 {
        match self {
            MatchResult::Lose => 0,
            MatchResult::Draw => 3,
            MatchResult::Win => 6,
        }
    }
}

impl Shape {
    fn to_number(self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Error, Debug)]
pub enum MatchError {
    #[error("`{0}` is not a match")]
    InvalidMatch(String),
    #[error("unknown shape error")]
    InvalidShape(ShapeError),
}

struct Match {
    player1: Shape,
    player2: Shape,
}

impl FromStr for Match {
    type Err = MatchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shapes: Vec<Result<Shape, ShapeError>> =
            s.split(' ').map(|s| s.parse::<Shape>()).collect();

        let Some(Ok(shape1)) = shapes.get(0) else {
            return Err(MatchError::InvalidMatch(s.to_string()))
        };

        let Some(Ok(shape2)) = shapes.get(1) else {
            return Err(MatchError::InvalidMatch(s.to_string()))
        };

        Ok(Match {
            player1: *shape1,
            player2: *shape2,
        })
    }
}

impl Match {
    fn fight(&self) -> MatchResult {
        match self.player1 {
            Shape::Rock => match self.player2 {
                Shape::Rock => MatchResult::Draw,
                Shape::Paper => MatchResult::Win,
                Shape::Scissors => MatchResult::Lose,
            },
            Shape::Paper => match self.player2 {
                Shape::Rock => MatchResult::Lose,
                Shape::Paper => MatchResult::Draw,
                Shape::Scissors => MatchResult::Win,
            },
            Shape::Scissors => match self.player2 {
                Shape::Rock => MatchResult::Win,
                Shape::Paper => MatchResult::Lose,
                Shape::Scissors => MatchResult::Draw,
            },
        }
    }

    fn to_number(&self) -> u32 {
        self.fight().to_number() + self.player2.to_number()
    }
}

// cargo run --release < inputs/2.txt --bin day2-1

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = "A Y\nB X\nC Z";

    #[test]
    fn score() {
        let matches: Vec<Match> = TEST_DATA
            .lines()
            .map(|l| l.parse::<Match>().unwrap())
            .collect();
        let scores: Vec<_> = matches.iter().map(|m| m.to_number()).collect();
        assert_eq!(scores, vec![8, 1, 6]);
    }
}

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
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err(ShapeError::InvalidShape(s.to_string())),
        }
    }
}

enum MatchResult {
    Lose,
    Draw,
    Win,
}

#[derive(Error, Debug)]
pub enum MatchResultError {
    #[error("`{0}` is not a match result")]
    InvalidMatchResult(String),
}

impl FromStr for MatchResult {
    type Err = MatchResultError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MatchResult::Lose),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(MatchResultError::InvalidMatchResult(s.to_string())),
        }
    }
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
    #[error("unknown match result error")]
    InvalidMatchResult(MatchResultError),
}

struct Match {
    player1: Shape,
    result: MatchResult,
}

impl FromStr for Match {
    type Err = MatchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let match_components: Vec<&str> = s.split(' ').collect();

        let Some(shape) = match_components.get(0) else {
            return Err(MatchError::InvalidMatch(s.to_string()))
        };

        let Ok(shape1) = shape.parse::<Shape>() else {
            return Err(MatchError::InvalidShape(ShapeError::InvalidShape(s.to_string())))
        };

        let Some(match_res) = match_components.get(1) else {
            return Err(MatchError::InvalidMatch(s.to_string()))
        };

        let Ok(match_res1) = match_res.parse::<MatchResult>() else {
            return Err(MatchError::InvalidMatchResult(MatchResultError::InvalidMatchResult(s.to_string())))
        };

        Ok(Match {
            player1: shape1,
            result: match_res1,
        })
    }
}

impl Match {
    fn guess_player2(&self) -> Shape {
        match self.player1 {
            Shape::Rock => match self.result {
                MatchResult::Draw => Shape::Rock,
                MatchResult::Win => Shape::Paper,
                MatchResult::Lose => Shape::Scissors,
            },
            Shape::Paper => match self.result {
                MatchResult::Draw => Shape::Paper,
                MatchResult::Win => Shape::Scissors,
                MatchResult::Lose => Shape::Rock,
            },
            Shape::Scissors => match self.result {
                MatchResult::Draw => Shape::Scissors,
                MatchResult::Win => Shape::Rock,
                MatchResult::Lose => Shape::Paper,
            },
        }
    }

    fn to_number(&self) -> u32 {
        self.result.to_number() + self.guess_player2().to_number()
    }
}

// cargo run --release < inputs/2.txt --bin day2-2

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
        assert_eq!(scores, vec![4, 1, 7]);
    }
}

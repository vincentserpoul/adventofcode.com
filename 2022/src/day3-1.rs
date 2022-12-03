#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    clippy::cargo
)]

use eyre::Result;
use std::io::{self, Read};
use std::str::FromStr;
use thiserror::Error;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let rucksacks = input
        .lines()
        .map(|l| l.parse::<Rucksack>().unwrap())
        .collect::<Vec<Rucksack>>();

    let commons: Vec<char> = rucksacks
        .iter()
        .filter_map(|r| r.find_common_item())
        .collect();

    println!("{}", value(commons));

    Ok(())
}

#[derive(Clone)]
struct Rucksack {
    compartment_1: Vec<char>,
    compartment_2: Vec<char>,
}

#[derive(Error, Debug)]
pub enum RucksackError {
    #[error("`{0}` is not a rucksack")]
    InvalidRucksack(String),
}

impl FromStr for Rucksack {
    type Err = RucksackError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (compartment_1, compartment_2) = s.split_at(s.len() / 2);

        Ok(Rucksack {
            compartment_1: compartment_1.chars().collect(),
            compartment_2: compartment_2.chars().collect(),
        })
    }
}

impl Rucksack {
    fn find_common_item(&self) -> Option<char> {
        self.compartment_1
            .iter()
            .find(|&c| self.compartment_2.contains(c))
            .copied()
    }
}

fn value(rucksack_common: Vec<char>) -> u32 {
    rucksack_common
        .iter()
        .map(|c| match c {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            'A' => 27,
            'B' => 28,
            'C' => 29,
            'D' => 30,
            'E' => 31,
            'F' => 32,
            'G' => 33,
            'H' => 34,
            'I' => 35,
            'J' => 36,
            'K' => 37,
            'L' => 38,
            'M' => 39,
            'N' => 40,
            'O' => 41,
            'P' => 42,
            'Q' => 43,
            'R' => 44,
            'S' => 45,
            'T' => 46,
            'U' => 47,
            'V' => 48,
            'W' => 49,
            'X' => 50,
            'Y' => 51,
            'Z' => 52,
            _ => 0,
        })
        .sum()
}

// cargo run --release < inputs/3.txt --bin day3-1

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn rucksack_commons() {
        let rucksacks = TEST_DATA
            .lines()
            .map(|l| l.parse::<Rucksack>().unwrap())
            .collect::<Vec<Rucksack>>();

        let commons: Vec<char> = rucksacks
            .iter()
            .filter_map(|r| r.find_common_item())
            .collect();

        assert_eq!(commons, vec!['p', 'L', 'P', 'v', 't', 's']);
    }

    #[test]
    fn score() {
        let rucksacks = TEST_DATA
            .lines()
            .map(|l| l.parse::<Rucksack>().unwrap())
            .collect::<Vec<Rucksack>>();

        let commons: Vec<char> = rucksacks
            .iter()
            .filter_map(|r| r.find_common_item())
            .collect();

        assert_eq!(value(commons), 157);
    }
}

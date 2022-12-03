#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    clippy::cargo
)]

use eyre::Result;
use std::io::{self, Read};

use std::collections::BinaryHeap;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut calories_heap = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<BinaryHeap<i32>>();

    let mut top_three: Vec<i32> = Vec::new();
    for _ in 0..3 {
        if let Some(v) = calories_heap.pop() {
            top_three.push(v);
        }
    }

    println!("{}", top_three.iter().sum::<i32>());

    Ok(())
}

// cargo run --release < inputs/1.txt --bin day1-2

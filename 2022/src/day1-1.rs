#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    clippy::cargo
)]

use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let max = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| l.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("{}", max);

    Ok(())
}

// cargo run --release < 1.txt --bin day1.1

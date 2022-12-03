#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    clippy::cargo
)]

use eyre::Result;
use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut groups = Vec::new();
    let mut lines = input.lines();

    while let (Some(rs_1), Some(rs_2), Some(rs_3)) = (lines.next(), lines.next(), lines.next()) {
        let hs_1 = rs_1.chars().collect::<HashSet<char>>();
        let hs_2 = rs_2.chars().collect::<HashSet<char>>();
        let hs_3 = rs_3.chars().collect::<HashSet<char>>();

        let first_inter = hs_1.intersection(&hs_2).copied().collect::<HashSet<_>>();

        let group_item = hs_3
            .intersection(&first_inter)
            .copied()
            .collect::<HashSet<_>>();

        let item = group_item.iter().next().unwrap();
        groups.push(*item);
    }

    println!("{:?}", groups.iter().map(|&x| value(x)).sum::<u32>());

    Ok(())
}

fn value(c: char) -> u32 {
    match c {
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
    }
}

// cargo run --release < inputs/3.txt --bin day3-1

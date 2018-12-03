use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    println!("checksum: {}", part1());
    Ok(())
}

fn part1() -> i32 {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut twos = 0;
    let mut threes = 0;

    BufReader::new(reader)
        .lines()
        .map(|l| {
            let mut letter_freq = HashMap::new();
            l.unwrap().chars().for_each(|ch| {
                // https://www.reddit.com/r/rust/comments/5nyi22/borrowed_temporary_value_does_not_live_long_enough/
                let count = letter_freq.entry(ch.to_string()).or_insert(0);
                *count += 1;
            });
            let mut contains_two = false;
            let mut contains_three = false;
            letter_freq.iter().for_each(|v| {
                if v.1 == &2 {
                    contains_two = true;
                }
                if v.1 == &3 {
                    contains_three = true;
                }
            });
            (contains_two, contains_three)
        }).for_each(|(contains_two, contains_three)| {
            if contains_two {
                twos += 1;
            }
            if contains_three {
                threes += 1;
            }
        });

    twos * threes
}

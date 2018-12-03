use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    println!("last freq: {}", part1());
    println!("first repeated freq: {}", part2());
    Ok(())
}

fn part1() -> i32 {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    BufReader::new(reader)
        .lines()
        .fold(0i32, |sum, l| sum + l.unwrap().parse::<i32>().unwrap())
}

fn part2() -> i32 {
    let mut freqs = HashSet::new();
    let mut curr_freq = 0;

    loop {
        let f = File::open("input.txt").unwrap();
        let reader = BufReader::new(f);
        for l in BufReader::new(reader).lines() {
            curr_freq += l.unwrap().parse::<i32>().unwrap();
            if freqs.contains(&curr_freq) {
                return curr_freq;
            }
            freqs.insert(curr_freq);
        }
    }
}

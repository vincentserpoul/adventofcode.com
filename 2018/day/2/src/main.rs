use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    println!("checksum: {}", part1());
    println!(
        "similar boxes: {}",
        match part2() {
            Some(s) => s,
            None => "no match found".to_string(),
        }
    );
    Ok(())
}

fn part1() -> i32 {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut twos = 0;
    let mut threes = 0;

    reader
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

fn part2() -> Option<String> {
    let f = File::open("input.txt").unwrap();

    for l in BufReader::new(f).lines().map(|l| l.unwrap()) {
        if let Some(s) = find_one_char_matching_box(&l) {
            return Some(get_common_chars(&s, &l));
        }
    }

    None
}

fn find_one_char_matching_box(a: &str) -> Option<String> {
    let f = File::open("input.txt").unwrap();
    BufReader::new(f)
        .lines()
        .map(|o| o.unwrap())
        .find(|o| are_one_char_diff(&a, &o))
}

fn are_one_char_diff(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let x = a.chars().zip(b.chars()).fold(0, |acc, (f, g)| {
        if f == g {
            return acc + 1;
        }
        acc
    });
    if x == a.len() - 1 {
        return true;
    }
    false
}

fn get_common_chars(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|(ac, bc)| ac == bc)
        .map(|x| x.0)
        .collect()
}

#[test]
fn test_get_common_chars() {
    assert_eq!(
        get_common_chars(&String::from("te"), &String::from("tes")),
        "te"
    );
}

#[test]
fn test_are_one_char_diff() {
    assert!(are_one_char_diff(
        &String::from("tev"),
        &String::from("tes")
    ));
}

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

use regex::Regex;

fn main() -> io::Result<()> {
    // let f = File::open("input.txt").unwrap();
    let f = File::open("./src/test_input.txt").unwrap();

    let mut deps: HashMap<char, Deps> = HashMap::new();

    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| Step::from_str(&l).unwrap())
        .for_each(|s| {
            deps.entry(s.letter)
                .and_modify(|d| {
                    d.follows.push(s.depends_on);
                })
                .or_insert(Deps {
                    precedes: Vec::new(),
                    follows: vec![s.depends_on],
                });

            deps.entry(s.depends_on)
                .and_modify(|d| {
                    d.precedes.push(s.letter);
                })
                .or_insert(Deps {
                    precedes: vec![s.letter],
                    follows: Vec::new(),
                });
        });

    // re-sorting for each letter
    deps.iter_mut().for_each(|(_k, v)| {
        v.follows.sort();
        v.precedes.sort();
    });

    let mut chain: Vec<char> = Vec::new();

    loop {
        // find next possible step
        let mut potential_next = deps
            .iter()
            .filter(|(_k, v)| v.follows.len() == 0)
            .filter(|(k, _v)| chain.iter().find(|c| c == k).is_none())
            .map(|(k, _v)| *k)
            .collect::<Vec<char>>();

        if potential_next.len() == 0 {
            break;
        }

        potential_next.sort();
        chain.push(potential_next[0]);

        // remove this letter from all the follows
        deps.iter_mut().for_each(|(_k, v)| {
            v.follows.retain(|l| *l != potential_next[0]);
        });
    }

    let ch = chain.iter().fold(String::from(""), |mut c, e| {
        c.push(*e);
        c
    });

    println!("{}", ch);

    Ok(())
}

#[derive(Debug)]
struct Deps {
    precedes: Vec<char>,
    follows: Vec<char>,
}

#[derive(PartialEq, Debug)]
struct Step {
    letter: char,
    depends_on: char,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "Step (?P<depends_on>[A-Z]) must be finished before step (?P<letter>[A-Z]) can begin."
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        Ok(Step {
            letter: caps["letter"].parse().unwrap(),
            depends_on: caps["depends_on"].parse().unwrap(),
        })
    }
}

#[test]
fn test_step() {
    assert_eq!(
        Step::from_str("Step P must be finished before step Z can begin."),
        Ok(Step {
            letter: 'Z',
            depends_on: 'P'
        })
    );
}

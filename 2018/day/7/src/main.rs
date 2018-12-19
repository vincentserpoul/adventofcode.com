#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

use regex::Regex;

const WORKERS_COUNT: usize = 5;
const MIN_S_STEP: usize = 60;

// const WORKERS_COUNT: usize = 2;
// const MIN_S_STEP: usize = 0;

fn main() -> io::Result<()> {
    let f = File::open("input.txt").unwrap();
    // let f = File::open("./src/test_input.txt").unwrap();

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
    let mut chain_processing: HashSet<char> = HashSet::new();

    let mut workers: [Option<CurrStep>; WORKERS_COUNT] = [None; WORKERS_COUNT];
    let mut seconds_passed = 0;

    loop {
        // Remove steps from ended worker
        workers.iter_mut().filter(|w| w.is_some()).for_each(|w| {
            let wu = w.unwrap();
            if wu.freed_at == seconds_passed {
                // pushing in result chain
                chain.push(wu.letter);

                // remove this letter from all the follows
                deps.iter_mut().for_each(|(_k, v)| {
                    v.follows.retain(|l| *l != wu.letter);
                });

                // remove this letter from the processing
                chain_processing.remove(&wu.letter);

                // remove work from worker
                *w = None;
            }
        });

        // find the free workers
        let free_worker_idxs = workers
            .iter()
            .enumerate()
            .filter(|(_, w)| w.is_none())
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        for free_worker_idx in free_worker_idxs {
            // find next possible step
            let mut potential_next = deps
                .iter()
                .filter(|(k, v)| {
                    v.follows.is_empty()
                        && !chain_processing.contains(k)
                        && chain.iter().find(|c| c == k).is_none()
                })
                .map(|(k, _v)| *k)
                .collect::<Vec<char>>();

            if !potential_next.is_empty() {
                potential_next.sort();

                // Lock the step to the free worker
                workers[free_worker_idx] = Some(CurrStep::new(potential_next[0], seconds_passed));

                chain_processing.insert(potential_next[0]);
            }
        }

        if deps
            .iter()
            .filter(|(k, v)| {
                v.follows.is_empty()
                    && !chain_processing.contains(k)
                    && chain.iter().find(|c| c == k).is_none()
            })
            .map(|(k, _v)| *k)
            .next()
            .is_none()
            && chain_processing.is_empty()
        {
            break;
        }

        print!("{}s: ", seconds_passed);
        workers.iter().for_each(|w| {
            print!(
                "{}",
                w.unwrap_or(CurrStep {
                    letter: '.',
                    freed_at: 0
                })
                .letter
            )
        });
        println!();

        seconds_passed += 1;
    }

    let ch = chain.iter().fold(String::from(""), |mut c, e| {
        c.push(*e);
        c
    });

    println!("it took {}s to do {}", seconds_passed, ch);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct CurrStep {
    letter: char,
    freed_at: usize,
}

impl CurrStep {
    fn new(letter: char, curr_sec: usize) -> CurrStep {
        CurrStep {
            letter,
            freed_at: curr_sec + step_to_seconds(letter),
        }
    }
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

fn step_to_seconds(c: char) -> usize {
    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    alpha.find(c).unwrap() + 1 + MIN_S_STEP
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

#[test]
fn test_step_to_seconds() {
    assert_eq!(step_to_seconds('A'), 1 + MIN_S_STEP);
    assert_eq!(step_to_seconds('Z'), 26 + MIN_S_STEP);
}

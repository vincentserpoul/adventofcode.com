use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::usize::MAX;

struct Polymer {
    ignored_unit_type: Option<char>,
    units: Vec<Unit>,
}

struct Unit {
    t: char,
    is_positive: bool,
}

impl Unit {
    fn new(c: char) -> Unit {
        Unit {
            t: c.to_lowercase().next().unwrap(),
            is_positive: c.is_uppercase(),
        }
    }

    fn does_react_with(&self, u: &Unit) -> bool {
        if self.t == u.t && self.is_positive != u.is_positive {
            return true;
        }
        false
    }
}

impl Polymer {
    fn new(iut: Option<char>) -> Polymer {
        Polymer {
            ignored_unit_type: iut,
            units: Vec::new(),
        }
    }

    fn add_unit(&mut self, c: char) {
        // Process new unit
        let nu = Unit::new(c);

        // Return early if it is an ignored unit type
        if Some(nu.t) == self.ignored_unit_type {
            return;
        }

        // Get last pushed unit
        let lu = match self.units.last() {
            Some(u) => u,
            None => {
                self.units.push(nu);
                return;
            }
        };

        // if they react together, remove the last unit
        if lu.does_react_with(&nu) {
            self.units.pop();
            return;
        }

        // if they don't react together, push it
        self.units.push(nu);
    }
}

fn main() -> io::Result<()> {
    let mut min_len = MAX;
    let mut min_len_char = 'a';
    for alpha in ("abcdefghijklmnopqrstuvwxyz").chars() {
        let mut p = Polymer::new(Some(alpha));
        let f = File::open("input.txt").unwrap();
        BufReader::new(f)
            .lines()
            .map(|l| l.unwrap())
            .for_each(|l| l.chars().for_each(|c| p.add_unit(c)));
        if p.units.len() < min_len {
            min_len = p.units.len();
            min_len_char = alpha;
        }
    }
    println!("{} for {}", min_len, min_len_char);

    Ok(())
}

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

struct Polymer {
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
    fn new() -> Polymer {
        Polymer { units: Vec::new() }
    }

    fn add_unit(&mut self, c: char) {
        // Process new unit
        let nu = Unit::new(c);

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
    let f = File::open("input.txt").unwrap();
    let mut p = Polymer::new();
    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .for_each(|l| l.chars().for_each(|c| p.add_unit(c)));

    println!("{}", p.units.len());
    Ok(())
}

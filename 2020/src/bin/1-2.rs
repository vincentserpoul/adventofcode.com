use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut set: HashSet<i32> = HashSet::new();
    for l in input.lines() {
        let n = l.parse::<i32>().unwrap();
        set.insert(n);
    }

    let found2 = find_sum(2020, 2, Vec::new(), Vec::new(), &set);

    found2.iter().for_each(|v| {
        println!("{} {} {}", v[0], v[1], v[0] * v[1]);
    });

    let found3 = find_sum(2020, 3, Vec::new(), Vec::new(), &set);

    found3.iter().for_each(|v| {
        println!("{} {} {}, {}", v[0], v[1], v[2], v[0] * v[1] * v[2]);
    });

    Ok(())
}

fn find_sum(
    target_sum: i32,
    target_size: usize,
    curr: Vec<i32>,
    mut found: Vec<Vec<i32>>,
    set: &HashSet<i32>,
) -> Vec<Vec<i32>> {
    if curr.iter().sum::<i32>() == target_sum && curr.len() >= target_size {
        found.push(curr);
        return found;
    }

    if curr.iter().sum::<i32>() > target_sum || curr.len() == target_size {
        return found;
    }

    for l in set {
        let mut new_curr = curr.clone();
        new_curr.push(*l);
        found = find_sum(target_sum, target_size, new_curr, found, set);
    }

    found
}

// cargo run --release < 1.txt --bin 1.2

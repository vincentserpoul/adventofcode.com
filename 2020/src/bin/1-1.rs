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
        if set.contains(&(2020 - n)) {
            println!("{} {} {}", n, 2020 - n, n * (2020 - n));
            return Ok(());
        }
    }

    Ok(())
}

// cargo run --release < 1.txt --bin 1.1

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let count_valid_passes = include_str!("../../inputs/2.txt")
        .split('\n')
        .filter(|s| {
            let v = s.split(": ").collect::<Vec<&str>>();
            if v.len() != 2 {
                return false;
            }
            let rule: Rule = Rule::from(v[0]);
            rule.is_valid(v[1])
        })
        .count();

    println!("{:}", count_valid_passes);

    Ok(())
}

use std::ops::RangeInclusive;

#[derive(Debug)]
struct Rule {
    range: RangeInclusive<usize>,
    c: char,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let (range, c): (&str, char) = match s.split_whitespace().collect::<Vec<&str>>() {
            x if x.len() == 2 && x[1].chars().count() == 1 => (x[0], x[1].chars().next().unwrap()),
            _ => ("0", '0'),
        };

        let (min_s, max_s) = match range.split('-').collect::<Vec<&str>>() {
            x if x.len() == 2 => (x[0], x[1]),
            _ => ("1", "1"),
        };

        let min = min_s.parse::<usize>().unwrap();
        let max = max_s.parse::<usize>().unwrap();

        if min > max {
            panic!("min > max")
        };

        Rule {
            range: min..=max,
            c,
        }
    }
}

impl Rule {
    fn is_valid(&self, password: &str) -> bool {
        // println!("{:?} is_valid({:})", self, password);
        let count_c = password.chars().filter(|x| x == &self.c).count();
        self.range.contains(&count_c)
    }
}

// cargo run --release --bin 2.1

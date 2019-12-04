use std::error::Error;
use std::i32;
use std::io::{self, Read};
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let vals: Vec<i32> = input
        .split('-')
        .take(2)
        .map(|v| v.parse().unwrap())
        .collect();

    println!("{:?}", find_pass_count(vals[0], vals[1]));

    Ok(())
}

fn find_pass_count(lower_bound: i32, upper_bound: i32) -> i32 {
    let mut count = 0;
    for i in lower_bound..upper_bound {
        if contains_exactly_one_double(i) && always_increasing(i) {
            // println!("{:?}", i);
            count += 1;
        }
    }
    count
}

fn contains_double(i: i32) -> bool {
    i.to_string()
        .chars()
        .fold((' ', false), |mut acc, c| {
            if c == acc.0 {
                acc.1 = true;
            }
            acc.0 = c;
            (c, acc.1)
        })
        .1
}

fn contains_exactly_one_double(i: i32) -> bool {
    let pass: Vec<char> = i.to_string().chars().collect();
    if pass.len() < 2 {
        return false;
    }

    let mut cur_idx: usize = 0;

    loop {
        if cur_idx + 1 > pass.len() - 1 {
            return false;
        }
        if pass[cur_idx] == pass[cur_idx + 1] {
            if cur_idx + 2 == pass.len() {
                return true;
            }
            if pass[cur_idx + 2] != pass[cur_idx] {
                return true;
            }
            if let Some(idx) = pass
                .iter()
                .skip(cur_idx + 2)
                .position(|x| *x != pass[cur_idx])
            {
                // println!("was {:?} nxt is {:?}", cur_idx, idx + cur_idx + 2);
                cur_idx += idx + 2;
            } else {
                return false;
            }
        } else {
            cur_idx += 1;
            if cur_idx > pass.len() - 2 {
                return false;
            }
        }
    }
}

fn always_increasing(i: i32) -> bool {
    i.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .fold((-1, true), |mut acc, c| {
            if c < acc.0 {
                acc.1 = false;
            }
            acc.0 = c;
            (c, acc.1)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pass_count() {
        assert_eq!(find_pass_count(10, 21), 1);
        assert_eq!(find_pass_count(122, 123), 1);
        assert_eq!(find_pass_count(1222, 1223), 0);
        assert_eq!(find_pass_count(2220, 2223), 0);
        assert_eq!(find_pass_count(232_220, 232_223), 0);
        assert_eq!(find_pass_count(278_889, 278_890), 0);
    }
}

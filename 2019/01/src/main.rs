use math::round::floor;
use std::error::Error;
use std::i32;
use std::io::{self, Read};
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!(
        "{}",
        input
            .lines()
            .map(|line| line.parse().unwrap())
            .map(calc_fuel_including_fuel)
            .sum::<i32>()
    );

    Ok(())
}

fn calc_fuel_including_fuel(mass: i32) -> i32 {
    let f = calc_fuel(mass);
    if f < 0 {
        return 0;
    }

    f + calc_fuel_including_fuel(f)
}

fn calc_fuel(mass: i32) -> i32 {
    floor((mass / 3).into(), 0) as i32 - 2
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calc_fuel_including_fuel() {
        assert_eq!(calc_fuel_including_fuel(14), 2);
    }
}

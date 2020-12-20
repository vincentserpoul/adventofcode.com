use std::collections::HashMap;

type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> ResultMain<()> {
    let mut cs = ComputerSystem((0, 0), HashMap::new());

    include_str!("../../inputs/14.txt")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|instruction| Instruction::from_str(instruction).unwrap())
        .for_each(|i| cs.apply_instruction(&i));

    println!(
        "ended up at ({:#?})",
        cs.1.iter().map(|(_a, v)| *v).sum::<u64>()
    );
    Ok(())
}

struct ComputerSystem((u64, u64), HashMap<u64, u64>);

impl ComputerSystem {
    fn apply_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Mask(set, clear) => self.0 = (*set, *clear),
            Instruction::Mem(add, val) => {
                let new_val = apply_mask(self.0, *val);
                self.1.insert(*add, new_val);
            }
        }
    }
}

fn apply_mask(mask: (u64, u64), mut val: u64) -> u64 {
    val |= mask.0;
    val &= mask.1;
    val
}

#[derive(std::clone::Clone, Debug, PartialEq)]
enum Instruction {
    Mask(u64, u64),
    Mem(u64, u64),
}

use std::str::FromStr;

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m: Vec<&str> = s.split(" = ").collect();
        if m.len() != 2 {
            return Err(());
        }

        if m[0].starts_with("mask") {
            let (set_mask, clear_mask) = m[1].chars().rev().enumerate().fold(
                (0_u64, std::u64::MAX),
                |mut acc, b: (usize, char)| match b.1 {
                    '1' => {
                        acc.0 |= 1 << b.0;
                        acc
                    }
                    '0' => {
                        acc.1 &= !(1 << b.0);
                        acc
                    }
                    _ => acc,
                },
            );

            return Ok(Instruction::Mask(set_mask, clear_mask));
        }

        if m[0].starts_with("mem") {
            let val = m[1].parse::<u64>().unwrap();
            let add = m[0]
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("not a number {}", s));

            return Ok(Instruction::Mem(add, val));
        }

        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intruction() {
        assert_eq!(
            Instruction::from_str("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX10X").unwrap(),
            Instruction::Mask(4, 18446744073709551613),
        );
        assert_eq!(
            Instruction::from_str("mem[805] = 29195").unwrap(),
            Instruction::Mem(805, 29195),
        );
    }
}

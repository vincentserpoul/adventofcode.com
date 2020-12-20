use std::collections::HashMap;

type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> ResultMain<()> {
    let mut cs = ComputerSystem((0, 0, "".to_string()), HashMap::new());

    include_str!("../../inputs/14.txt")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|instruction| Instruction::from_str(instruction).unwrap())
        .for_each(|i| cs.apply_instruction(&i));

    println!(
        "ended up at ({:#?})",
        cs.1.iter().map(|(_a, v)| *v).sum::<u64>()
    );

    let mut cs2 = ComputerSystem((0, 0, "".to_string()), HashMap::new());
    include_str!("../../inputs/14.txt")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|instruction| Instruction::from_str(instruction).unwrap())
        .for_each(|i| cs2.apply_instruction_v2(&i));

    println!(
        "ended up at ({:#?})",
        cs2.1.iter().map(|(_a, v)| *v).sum::<u64>()
    );

    Ok(())
}

struct ComputerSystem((u64, u64, String), HashMap<u64, u64>);

impl ComputerSystem {
    fn apply_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Mask(set, clear, full) => self.0 = (*set, *clear, full.to_owned()),
            Instruction::Mem(add, val) => {
                let new_val = apply_mask(self.0.clone(), *val);
                self.1.insert(*add, new_val);
            }
        }
    }

    fn apply_instruction_v2(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Mask(set, clear, full) => self.0 = (*set, *clear, full.to_owned()),
            Instruction::Mem(add, val) => {
                let all_mems = apply_mask_v2(&self.0, *add);
                all_mems.iter().for_each(|a| {
                    self.1.insert(*a, *val);
                });
            }
        }
    }
}

fn mems(mem: &str) -> Vec<u64> {
    if !mem.contains('X') {
        return vec![u64::from_str_radix(mem, 2).unwrap()];
    }

    let mut mem0 = mems(mem.replacen("X", "0", 1).as_ref());
    let mem1 = mems(mem.replacen("X", "1", 1).as_ref());

    mem0.extend(mem1);

    mem0
}

fn apply_mask(mask: (u64, u64, String), mut val: u64) -> u64 {
    val |= mask.0;
    val &= mask.1;
    val
}

fn apply_mask_v2(mask: &(u64, u64, String), mem: u64) -> Vec<u64> {
    let mem_b = format!("{:0>36}", format!("{:b}", mem));

    let masked_mem = mem_b
        .chars()
        .zip(mask.2.chars())
        .fold("".to_string(), |mut acc, res| {
            let r = match res {
                ('0', '0') => '0',
                ('0', '1') => '1',
                ('1', '0') => '1',
                ('1', '1') => '1',
                ('0', 'X') => 'X',
                ('1', 'X') => 'X',
                (a, b) => panic!("({}, {}) is not handled", a, b),
            };
            acc.push(r);
            acc
        });

    mems(masked_mem.as_str())
}

#[derive(std::clone::Clone, Debug, PartialEq)]
enum Instruction {
    Mask(u64, u64, String),
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

            return Ok(Instruction::Mask(set_mask, clear_mask, m[1].to_string()));
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
            Instruction::Mask(
                4,
                18446744073709551613,
                "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX10X".to_string()
            ),
        );
        assert_eq!(
            Instruction::from_str("mem[805] = 29195").unwrap(),
            Instruction::Mem(805, 29195),
        );
    }

    #[test]
    fn test_mems() {
        assert_eq!(mems("XX"), vec![0, 1, 2, 3]);
        assert_eq!(mems("0X"), vec![0, 1]);
        assert_eq!(mems("XXX"), vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_apply_mask_v2() {
        assert_eq!(
            apply_mask_v2(
                &(0, 0, "000000000000000000000000000000X1001X".to_string()),
                42
            ),
            vec![26, 27, 58, 59]
        );
    }
}

type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

use std::collections::HashSet;

fn main() -> ResultMain<()> {
    let mut hs = HashSet::<usize>::new();

    let instructions_iter = include_str!("../../inputs/8.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|i| Instruction::from_str(i).unwrap());

    let instructions = instructions_iter.clone().collect::<Vec<Instruction>>();

    let mut curr_instr = 0_i32;
    let mut res = 0_i32;
    while !hs.contains(&(curr_instr as usize)) && curr_instr < instructions.len() as i32 {
        hs.insert(curr_instr as usize);
        match instructions[curr_instr as usize] {
            Instruction::Acc(x) => {
                res += x;
                curr_instr += 1;
            }
            Instruction::Jmp(x) => curr_instr += x,
            Instruction::Nop(_x) => {
                curr_instr += 1;
            }
        }
    }

    println!("{}", res);

    // first swap
    let mut swap = 1;

    loop {
        println!("trying to swap {} occurrence of jmp/nop", swap);
        let mut occ = 0;
        let mut hs = HashSet::<usize>::new();

        let instructions = instructions_iter
            .clone()
            .map(|instr| match instr {
                Instruction::Jmp(x) => {
                    occ += 1;
                    if occ == swap {
                        return Instruction::Nop(x);
                    }

                    instr
                }
                Instruction::Nop(x) => {
                    occ += 1;
                    if occ == swap {
                        return Instruction::Jmp(x);
                    }

                    instr
                }
                _ => instr,
            })
            .collect::<Vec<Instruction>>();

        let mut curr_instr = 0_i32;
        let mut res = 0_i32;
        while !hs.contains(&(curr_instr as usize)) && curr_instr < instructions.len() as i32 {
            hs.insert(curr_instr as usize);
            match instructions[curr_instr as usize] {
                Instruction::Acc(x) => {
                    res += x;
                    curr_instr += 1;
                }
                Instruction::Jmp(x) => curr_instr += x,
                Instruction::Nop(_x) => {
                    curr_instr += 1;
                }
            }
        }
        println!("looped at instr {}", curr_instr);
        if curr_instr as usize == instructions.len() {
            println!("{}", res);
            return Ok(());
        }

        swap += 1;
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

use std::str::FromStr;

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: Vec<&str> = s.split_whitespace().collect();
        if vals.len() != 2 {
            return Err(());
        }
        match (vals[0], vals[1].parse::<i32>().unwrap()) {
            ("acc", x) => Ok(Instruction::Acc(x)),
            ("jmp", x) => Ok(Instruction::Jmp(x)),
            ("nop", x) => Ok(Instruction::Nop(x)),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_from_str() {
        assert_eq!(
            Instruction::from_str("jmp +4").unwrap(),
            Instruction::Jmp(4),
        );
        assert_eq!(
            Instruction::from_str("acc -300").unwrap(),
            Instruction::Acc(-300),
        );
    }
}

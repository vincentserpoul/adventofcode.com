use std::error::Error;
use std::io::{self, Read};
use std::result::Result;
use std::usize;

// reusing https://dev.to/mdenchev/the-aim-for-elegant-rust-advent-of-code-2019-problem-2-1jig as a cleaner basis

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let goal = 19_690_720;
    let inp = input
        .split(',')
        .map(|code| code.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    'noun: for noun in 0..=99usize {
        for verb in 0..=99usize {
            if run_prog(inp.clone(), noun, verb) == goal {
                println!("{}", 100 * noun + verb);
                break 'noun;
            }
        }
    }

    Ok(())
}

#[derive(PartialEq)]
enum OpCode {
    Add,
    Multiply,
    Halt,
    // SaveAt,
    // ReadAt,
}

impl From<usize> for OpCode {
    fn from(item: usize) -> OpCode {
        use OpCode::*;

        match item {
            1 => Add,
            2 => Multiply,
            // 3 => SaveAt,
            // 4 => ReadAt,
            99 => Halt,
            _ => unreachable!(),
        }
    }
}

fn run_prog(inp: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut inp = inp;
    let mut sp = 0usize;
    inp[1] = noun;
    inp[2] = verb;

    loop {
        let op = inp[sp].into();
        match op {
            OpCode::Add => {
                let p1 = inp[sp + 1];
                let p2 = inp[sp + 2];
                let p3 = inp[sp + 3];
                inp[p3] = inp[p1] + inp[p2];
                sp += 4;
            }
            OpCode::Multiply => {
                let p1 = inp[sp + 1];
                let p2 = inp[sp + 2];
                let p3 = inp[sp + 3];
                inp[p3] = inp[p1] * inp[p2];
                sp += 4;
            }
            OpCode::Halt => break,
        }
    }
    inp[0]
}

#[cfg(test)]
mod tests {
    use super::*;
}

use std::error::Error;
use std::i32;
use std::io::{self, Read};
use std::result::Result;
use std::vec::Vec;

const ADD: i32 = 1;
const MUL: i32 = 2;
const END: i32 = 99;

const WANTED: i32 = 19_690_720;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    'noun: for noun in 0..99 {
        for verb in 0..99 {
            let mut it = input
                .split(',')
                .map(|code| code.parse().unwrap())
                .collect::<Vec<i32>>();

            // let noun = 12;
            // let verb = 2;

            it[1] = noun;
            it[2] = verb;

            let mut cur_idx: usize = 0;
            'calc: loop {
                match it[cur_idx] {
                    END => {
                        if it[0] == WANTED {
                            println!("{}", noun * 100 + verb);
                            break 'noun;
                        }
                        break 'calc;
                    }
                    ADD => {
                        let left_idx = it[cur_idx + 1] as usize;
                        let right_idx = it[cur_idx + 2] as usize;
                        let res_idx = it[cur_idx + 3] as usize;
                        it[res_idx] = it[left_idx] + it[right_idx];
                        cur_idx += 4;
                    }
                    MUL => {
                        let left_idx = it[cur_idx + 1] as usize;
                        let right_idx = it[cur_idx + 2] as usize;
                        let res_idx = it[cur_idx + 3] as usize;
                        it[res_idx] = it[left_idx] * it[right_idx];
                        cur_idx += 4;
                    }
                    _ => {
                        break 'calc;
                    }
                }
            }
        }
    }

    Ok(())
}

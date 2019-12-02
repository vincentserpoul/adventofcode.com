use std::error::Error;
use std::i32;
use std::io::{self, Read};
use std::result::Result;
use std::vec::Vec;

const ADD: i32 = 1;
const MUL: i32 = 2;
const END: i32 = 99;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut it = input
        .split(',')
        .map(|code| code.parse().unwrap())
        .collect::<Vec<i32>>();

    let mut cur_idx: usize = 0;
    loop {
        match it[cur_idx] {
            END => {
                println!("found 99");
                break;
            }
            ADD => {
                let left_idx = it[cur_idx + 1] as usize;
                let right_idx = it[cur_idx + 2] as usize;
                let res_idx = it[cur_idx + 3] as usize;
                println!("updating {} with {}", res_idx, it[left_idx] + it[right_idx]);
                it[res_idx] = it[left_idx] + it[right_idx];
                cur_idx += 4;
            }
            MUL => {
                let left_idx = it[cur_idx + 1] as usize;
                let right_idx = it[cur_idx + 2] as usize;
                let res_idx = it[cur_idx + 3] as usize;
                println!("updating {} with {}", res_idx, it[left_idx] * it[right_idx]);
                it[res_idx] = it[left_idx] * it[right_idx];
                cur_idx += 4;
            }
            _ => {
                println!("_ found {}", it[cur_idx]);
                break;
            }
        }
    }

    println!("{:?}", it[0]);

    Ok(())
}

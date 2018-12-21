use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::result;

type Result<T> = result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    File::open("./input.txt")?.read_to_string(&mut s)?;

    let ns = s
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u32>>();
    let n = Node::from_slice(&ns)?;

    println!("{:?}", n.sum_metadata());
    println!("{:?}", n.sum_metadata_part2());

    Ok(())
}

#[derive(Debug)]
struct Node {
    metadata: Vec<u32>,
    children: Vec<Node>,
    size: usize,
}

impl Node {
    fn from_slice(sl: &[u32]) -> Result<Node> {
        // println!("{:?}", sl);
        let children_count = sl[0];
        let meta_count = sl[1];

        let mut n = Node {
            metadata: Vec::new(),
            children: Vec::new(),
            size: 2,
        };
        for _ in 0..children_count {
            let cn = Node::from_slice(&sl[n.size..])?;
            n.size += cn.size;
            n.children.push(cn);
        }

        for _ in 0..meta_count {
            n.metadata.push(*sl.get(n.size).unwrap());
            n.size += 1;
        }

        Ok(n)
    }

    fn sum_metadata(&self) -> u32 {
        let mut sum = self.metadata.iter().sum::<u32>();
        self.children.iter().for_each(|c| {
            sum += c.sum_metadata();
        });
        sum
    }

    fn sum_metadata_part2(&self) -> u32 {
        if self.children.len() == 0 {
            return self.metadata.iter().sum::<u32>();
        }

        self.metadata
            .iter()
            .filter(|m| self.children.get(**m as usize - 1).is_some())
            .map(|m| {
                self.children
                    .get(*m as usize - 1)
                    .unwrap()
                    .sum_metadata_part2()
            })
            .sum()
    }
}

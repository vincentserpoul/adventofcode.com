type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

use std::collections::HashSet;

fn main() -> ResultMain<()> {
    let sum_all_counts = include_str!("../../inputs/6.txt")
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| !c.is_whitespace())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum::<usize>();

    println!("sum all: {:}", sum_all_counts);

    let sum_intersects = include_str!("../../inputs/6.txt")
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|member| {
                    member
                        .chars()
                        .filter(|&c| !c.is_whitespace())
                        .collect::<HashSet<char>>()
                })
                .fold(('a'..='z').collect::<HashSet<char>>(), |acc, hs| {
                    acc.intersection(&hs).cloned().collect()
                })
                .len()
        })
        .sum::<usize>();

    println!("sum intersects: {:}", sum_intersects);
    Ok(())
}

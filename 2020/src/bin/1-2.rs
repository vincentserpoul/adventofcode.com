use std::collections::HashSet;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let set = include_str!("../../inputs/1.txt")
        .split('\n')
        .map(|it| it.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>();

    let found2 = find_sum(2020, 2, Vec::new(), Vec::new(), &set);

    found2.iter().for_each(|v| {
        println!("{} {} {}", v[0], v[1], v[0] * v[1]);
    });

    let found3 = find_sum(2020, 3, Vec::new(), Vec::new(), &set);

    found3.iter().for_each(|v| {
        println!("{} {} {}, {}", v[0], v[1], v[2], v[0] * v[1] * v[2]);
    });

    Ok(())
}

fn find_sum(
    target_sum: u32,
    target_size: usize,
    curr: Vec<u32>,
    mut found: Vec<Vec<u32>>,
    set: &HashSet<u32>,
) -> Vec<Vec<u32>> {
    if target_size == 0 || target_sum == 0 {
        return found;
    }

    if curr.iter().sum::<u32>() > target_sum || curr.len() == target_size {
        return found;
    }

    if curr.len() == target_size - 1 {
        let complement_sum = target_sum - curr.iter().sum::<u32>();
        if set.contains(&complement_sum) {
            let mut new_curr = curr;
            new_curr.push(complement_sum);
            found.push(new_curr);
        }
        return found;
    }

    for l in set {
        let mut new_curr = curr.clone();
        new_curr.push(*l);
        found = find_sum(target_sum, target_size, new_curr, found, set);
    }

    found
}

// cargo run --release --bin 1.2

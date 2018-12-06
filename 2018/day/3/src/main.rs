use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct Fabric {
    id: u32,
    dist_from_left: u32,
    dist_from_top: u32,
    width: u32,
    height: u32,
}

impl FromStr for Fabric {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(line_to_fabric(s).unwrap())
    }
}

impl fmt::Display for Fabric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Fabric #{} - left: {}, top: {}, dim: {}x{}",
            self.id, self.dist_from_left, self.dist_from_top, self.width, self.height
        )
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt").unwrap();
    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .for_each(|l| println!("{}", Fabric::from_str(&l).unwrap()));
    Ok(())
}

fn line_to_fabric(l: &str) -> Option<Fabric> {
    // #1 @ 1,3: 4x4
    let splitted = l
        .split(|c| c == ' ' || c == '#' || c == '@' || c == ',' || c == ':' || c == 'x')
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<u32>>();

    if splitted.len() != 5 {
        return None;
    }

    Some(Fabric {
        id: splitted[0],
        dist_from_left: splitted[1],
        dist_from_top: splitted[2],
        width: splitted[3],
        height: splitted[4],
    })
}

#[test]
fn test_line_to_fabric() {
    assert_eq!(
        line_to_fabric("#1 @ 1,3: 4x4"),
        Some(Fabric {
            id: 1,
            dist_from_left: 1,
            dist_from_top: 3,
            width: 4,
            height: 4,
        }),
    );
    assert_eq!(
        line_to_fabric("#2 @ 3,4: 5x6"),
        Some(Fabric {
            id: 2,
            dist_from_left: 3,
            dist_from_top: 4,
            width: 5,
            height: 6,
        }),
    );
}

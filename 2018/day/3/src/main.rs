use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

fn main() -> io::Result<()> {
    let f = File::open("input.txt").unwrap();
    let mut s = Solution::new();
    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .map(|s| Fabric::from_str(&s).unwrap())
        .for_each(|f| s.inject_into_canvas(&f));

    println!("{}", s.dupl_count);
    Ok(())
}

#[derive(PartialEq, Debug)]
struct Fabric {
    id: u32,
    dist_from_left: u32,
    dist_from_top: u32,
    width: u32,
    height: u32,
}

struct Solution {
    canvas: HashSet<(u32, u32)>,
    dupl_points: HashSet<(u32, u32)>,
    dupl_count: u32,
}

impl Solution {
    fn new() -> Solution {
        Solution {
            canvas: HashSet::new(),
            dupl_points: HashSet::new(),
            dupl_count: 0,
        }
    }

    fn inject_into_canvas(&mut self, f: &Fabric) {
        (f.dist_from_top..(f.dist_from_top + f.height)).for_each(|y| {
            (f.dist_from_left..(f.width + f.dist_from_left)).for_each(|x| {
                // if point has already been covered
                if !self.canvas.insert((x, y)) {
                    // if point was never counted
                    if self.dupl_points.insert((x, y)) {
                        self.dupl_count += 1;
                    }
                }
            })
        });
    }
}

impl FromStr for Fabric {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(line_to_fabric(s).unwrap())
    }
}

impl Display for Fabric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Fabric #{} - left: {}, top: {}, dim: {}x{}",
            self.id, self.dist_from_left, self.dist_from_top, self.width, self.height
        )
    }
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

#[test]
fn test_inject_into_canvas_0() {
    let mut s = Solution::new();
    s.inject_into_canvas(&line_to_fabric("#1 @ 1,3: 4x4").unwrap());
    s.inject_into_canvas(&line_to_fabric("#2 @ 3,1: 4x4").unwrap());
    s.inject_into_canvas(&line_to_fabric("#3 @ 5,5: 2x2").unwrap());

    assert_eq!(s.dupl_count, 4);
}

#[test]
fn test_inject_into_canvas_1() {
    let mut s = Solution::new();
    s.inject_into_canvas(&line_to_fabric("#1 @ 1,3: 4x4").unwrap());
    s.inject_into_canvas(&line_to_fabric("#2 @ 3,1: 4x4").unwrap());
    s.inject_into_canvas(&line_to_fabric("#3 @ 5,5: 2x2").unwrap());
    s.inject_into_canvas(&line_to_fabric("#4 @ 3,3: 2x2").unwrap());

    assert_eq!(s.dupl_count, 4);
}

use std::error::Error;
use std::i32;
use std::io::{self, Read};
use std::result::Result;

#[derive(Debug)]
struct Segment(Point, Point);

#[derive(Clone, Copy, Debug)]
struct Point(i32, i32);

fn wire_intersect(w1: Vec<Segment>, w2: Vec<Segment>) -> Option<Point> {
    Some(Point(0, 0))
}

fn segment_intersect(s1: &Segment, s2: &Segment) -> Option<Point> {
    Some(Point(0, 0))
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let wires = input
        .lines()
        .map(|line| {
            line.split(',')
                .fold(vec![Segment(Point(0, 0), Point(0, 0))], |mut acc, cmd| {
                    let (direction, distance_str) = cmd.split_at(1);
                    let distance: i32 = distance_str.parse().unwrap();
                    let last_point = acc.last().expect("no last segment").1;
                    let new_point = match direction {
                        "U" => Point(last_point.0 + distance, last_point.1),
                        "R" => Point(last_point.0, last_point.1 + distance),
                        "D" => Point(last_point.0 - distance, last_point.1),
                        "L" => Point(last_point.0, last_point.1 - distance),
                        _ => panic!("Haa, invalid command"),
                    };
                    acc.push(Segment(last_point, new_point));
                    acc
                })
        })
        .collect::<Vec<Vec<Segment>>>();

    println!("{:?} {:?}", wires[0], wires[1]);
    Ok(())
}

use std::error::Error;
use std::i32;
use std::io::{self, Read};
use std::result::Result;

#[derive(Clone, Copy, Debug)]
struct Segment {
    origin: Point,
    end: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn wire_intersect(w1: Vec<Segment>, w2: Vec<Segment>) -> (Option<Point>, i32) {
    let mut final_intersection: Option<Point> = None;
    let mut min_steps = i32::max_value();
    let central_port = Point { x: 0, y: 0 };

    for s1 in &w1 {
        for s2 in &w2 {
            if let Some(intersect) = segment_intersect(s1, s2) {
                if intersect != central_port
                    && steps(&w1, intersect) + steps(&w2, intersect) < min_steps
                {
                    min_steps = steps(&w1, intersect) + steps(&w2, intersect);
                    final_intersection = Some(intersect);
                }
            }
        }
    }

    (final_intersection, min_steps)
}

fn segment_intersect(s1: &Segment, s2: &Segment) -> Option<Point> {
    if s1.origin.x == s1.end.x
        && s2.origin.y == s2.end.y
        && belongs_to(s1.origin.x, s2.origin.x, s2.end.x)
        && belongs_to(s2.origin.y, s1.origin.y, s1.end.y)
    {
        return Some(Point {
            x: s1.origin.x,
            y: s2.origin.y,
        });
    }

    if s1.origin.y == s1.end.y
        && s2.origin.x == s2.end.x
        && belongs_to(s1.origin.y, s2.origin.y, s2.end.y)
        && belongs_to(s2.origin.x, s1.origin.x, s1.end.x)
    {
        return Some(Point {
            x: s2.origin.x,
            y: s1.origin.y,
        });
    }

    None
}

fn belongs_to(x: i32, a: i32, b: i32) -> bool {
    if a == b {
        return x == b;
    }
    if a > b {
        return x > b && x < a;
    }

    x > a && x < b
}

fn point_belong_to_segment(p: Point, s: Segment) -> bool {
    belongs_to(p.x, s.origin.x, s.end.x) && belongs_to(p.y, s.origin.y, s.end.y)
}

fn manhattan_distance(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn steps(w: &[Segment], p: Point) -> i32 {
    // println!(
    //     "{}",
    //     w.iter()
    //         .take_while(|s| !point_belong_to_segment(p, **s))
    //         .fold(0, |mut acc, s| {
    //             acc += manhattan_distance(s.origin, s.end);
    //             acc
    //         })
    // );
    // println!(
    //     "{}",
    //     manhattan_distance(
    //         w.iter()
    //             .find(|s| point_belong_to_segment(p, **s))
    //             .unwrap()
    //             .origin,
    //         p,
    //     )
    // );
    w.iter()
        .take_while(|s| !point_belong_to_segment(p, **s))
        .fold(0, |mut acc, s| {
            acc += manhattan_distance(s.origin, s.end);
            acc
        })
        + manhattan_distance(
            w.iter()
                .find(|s| point_belong_to_segment(p, **s))
                .unwrap()
                .origin,
            p,
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let wires = input
        .lines()
        .map(|line| {
            line.split(',')
                .fold(Vec::new(), |mut acc: Vec<Segment>, cmd| {
                    let (direction, distance_str) = cmd.split_at(1);
                    let distance: i32 = distance_str.parse().unwrap();
                    let last_point = acc
                        .last()
                        .unwrap_or(&Segment {
                            origin: Point { x: 0, y: 0 },
                            end: Point { x: 0, y: 0 },
                        })
                        .end;
                    let new_point = match direction {
                        "U" => Point {
                            x: last_point.x + distance,
                            y: last_point.y,
                        },
                        "R" => Point {
                            x: last_point.x,
                            y: last_point.y + distance,
                        },
                        "D" => Point {
                            x: last_point.x - distance,
                            y: last_point.y,
                        },
                        "L" => Point {
                            x: last_point.x,
                            y: last_point.y - distance,
                        },
                        _ => panic!("Haa, invalid command"),
                    };
                    acc.push(Segment {
                        origin: last_point,
                        end: new_point,
                    });
                    acc
                })
        })
        .collect::<Vec<Vec<Segment>>>();
    println!("{:?}", wire_intersect(wires[0].clone(), wires[1].clone()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_belongs_to() {
        assert_eq!(belongs_to(0, 3, 8), false);
    }

    #[test]
    fn test_steps_point() {
        assert_eq!(
            steps(
                &[Segment {
                    origin: Point { x: 0, y: 0 },
                    end: Point { x: 0, y: 0 }
                }],
                Point { x: 0, y: 0 }
            ),
            0
        );
    }

    #[test]
    fn test_steps_wire() {
        assert_eq!(
            steps(
                &[
                    Segment {
                        origin: Point { x: 0, y: 0 },
                        end: Point { x: 8, y: 0 }
                    },
                    Segment {
                        origin: Point { x: 8, y: 0 },
                        end: Point { x: 8, y: 5 }
                    },
                    Segment {
                        origin: Point { x: 8, y: 5 },
                        end: Point { x: 3, y: 5 }
                    },
                    Segment {
                        origin: Point { x: 3, y: 5 },
                        end: Point { x: 3, y: 2 }
                    },
                ],
                Point { x: 3, y: 3 }
            ),
            20
        );
    }
}

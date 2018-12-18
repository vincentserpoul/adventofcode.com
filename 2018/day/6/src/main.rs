use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    // let f = File::open("./src/test_input.txt").unwrap();
    let f = File::open("./input.txt").unwrap();
    let mut c = Canvas::new();
    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split(", ")
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i32>>()
        })
        .filter(|p| p.len() == 2)
        .for_each(|p| c.mark_point(p[0], p[1]));

    c.populate();

    // println!("{:?}", c.points);

    // println!("{:?}", c.excluded_areas);

    println!("{:?}", c.find_largest_finite_area());
    println!("{:?}", c.find_points_less_then(10000));

    Ok(())
}

#[derive(Debug)]
struct Canvas {
    marked_points: Vec<MarkedPoint>,
    points: Vec<Point>,
    max_x: i32,
    max_y: i32,
    excluded_areas: HashSet<usize>,
    marked_points_cum_dist: HashMap<usize, u32>,
}

impl Canvas {
    fn new() -> Canvas {
        Canvas {
            marked_points: Vec::new(),
            points: Vec::new(),
            max_x: 0,
            max_y: 0,
            excluded_areas: HashSet::new(),
            marked_points_cum_dist: HashMap::new(),
        }
    }

    fn mark_point(&mut self, x: i32, y: i32) {
        if x > self.max_x {
            self.max_x = x;
        }
        if y > self.max_y {
            self.max_y = y;
        }
        self.marked_points.push(MarkedPoint { x, y });
    }

    fn populate(&mut self) {
        // for each point, find the closest point
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let mut p = Point {
                    x,
                    y,
                    closest_point_id: Vec::new(),
                    point_dist: HashMap::new(),
                    dist: 0,
                };
                let min_dist = self
                    .marked_points
                    .iter()
                    .map(|mp| dist(&p, mp))
                    .min()
                    .unwrap();
                for (i, mp) in self.marked_points.iter().enumerate() {
                    let dist = dist(&p, mp);
                    if dist == min_dist {
                        p.closest_point_id.push(i);
                        p.dist = min_dist;
                    }
                    p.point_dist.insert(i, dist);
                }
                self.points.push(p);
            }
        }
        self.excluded_areas = self.find_infinite_areas();
    }

    fn find_infinite_areas(&mut self) -> HashSet<usize> {
        self.points
            .iter()
            .filter(|p| {
                (p.x == 0 || p.x == self.max_x || p.y == 0 || p.y == self.max_y)
                    && p.closest_point_id.len() == 1
            })
            .map(|p| p.closest_point_id[0])
            .fold(
                {
                    let hs: HashSet<usize> = HashSet::new();
                    hs
                },
                |mut hs, v| {
                    hs.insert(v);
                    hs
                },
            )
    }

    fn find_largest_finite_area(&mut self) -> u32 {
        // go through the points
        *self
            .points
            .iter()
            .filter(|p| {
                p.closest_point_id.len() == 1
                    && !self.excluded_areas.contains(&p.closest_point_id[0])
            })
            .fold(
                {
                    let h: HashMap<usize, u32> = HashMap::new();
                    h
                },
                |mut h, p| {
                    *h.entry(p.closest_point_id[0]).or_insert(0) += 1;
                    h
                },
            )
            .values()
            .max()
            .unwrap()
    }

    fn find_points_less_then(&self, limit_dist: i32) -> usize {
        self.points
            .iter()
            .filter(|p| {
                p.point_dist.iter().fold(0, |mut cum, (_k, d)| {
                    cum += d;
                    cum
                }) < limit_dist
            })
            .collect::<Vec<&Point>>()
            .len()
    }
}

#[derive(Debug)]
struct MarkedPoint {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point {
    closest_point_id: Vec<usize>,
    dist: i32,
    point_dist: HashMap<usize, i32>,
    x: i32,
    y: i32,
}

fn dist(a: &Point, b: &MarkedPoint) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

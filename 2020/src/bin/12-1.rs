type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> ResultMain<()> {
    let c = include_str!("../../inputs/12.txt")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|instruction| Instruction::from_str(instruction).unwrap())
        .fold(Coords(0, 0, Direction::E), |c, instr| {
            apply_instruction(&c, instr)
        });

    println!("ended up at ({:#?})", c);
    Ok(())
}

#[derive(Debug)]
struct Coords(i32, i32, Direction);

fn apply_instruction(c: &Coords, instr: Instruction) -> Coords {
    match instr {
        Instruction::Direction(dir, qty) => move_to_direction(c, &dir, qty),
        Instruction::Rotation(rot) => Coords(c.0, c.1, c.2.apply_rotation(rot)),
        Instruction::Forward(qty) => move_to_direction(c, &c.2, qty),
    }
}

fn move_to_direction(c: &Coords, direction: &Direction, qty: i32) -> Coords {
    match direction {
        Direction::N => Coords(c.0, c.1 + qty, c.2),
        Direction::E => Coords(c.0 + qty, c.1, c.2),
        Direction::S => Coords(c.0, c.1 - qty, c.2),
        Direction::W => Coords(c.0 - qty, c.1, c.2),
    }
}

enum Instruction {
    Direction(Direction, i32),
    Rotation(Rotation),
    Forward(i32),
}

use std::str::FromStr;

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let qty = s
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("not a number {}", s));

        if let Ok(dir) = Direction::from_str(s) {
            return Ok(Instruction::Direction(dir, qty));
        }
        if let Ok(rot) = Rotation::from_str(s) {
            return Ok(Instruction::Rotation(rot));
        }

        if s.chars().filter(|c| c.is_alphabetic()).collect::<String>() == "F" {
            return Ok(Instruction::Forward(qty));
        };

        Err(())
    }
}

#[derive(std::clone::Clone, Copy, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = s.chars().filter(|c| c.is_alphabetic()).collect::<String>();

        match instr.as_str() {
            "N" => Ok(Direction::N),
            "E" => Ok(Direction::E),
            "S" => Ok(Direction::S),
            "W" => Ok(Direction::W),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum Rotation {
    L(i32),
    R(i32),
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = s.chars().filter(|c| c.is_alphabetic()).collect::<String>();
        let qty = s
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("not a number {}", s));

        match instr.as_str() {
            "L" => Ok(Rotation::L(qty)),
            "R" => Ok(Rotation::R(qty)),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn apply_rotation(&self, rot: Rotation) -> Direction {
        let all_dirs = [Direction::N, Direction::E, Direction::S, Direction::W];
        let curr_dir_idx = match &self {
            Direction::N => 0,
            Direction::E => 1,
            Direction::S => 2,
            Direction::W => 3,
        };

        match rot {
            Rotation::R(x) => all_dirs[(curr_dir_idx + (x / 90) as usize) % 4],
            Rotation::L(x) => all_dirs[(curr_dir_idx - (x / 90) as usize) % 4],
        }
    }
}

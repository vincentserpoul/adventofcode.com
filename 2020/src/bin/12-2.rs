type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> ResultMain<()> {
    let c = include_str!("../../inputs/12.txt")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|instruction| Instruction::from_str(instruction).unwrap())
        .fold(Coords(10, 1, 0, 0), |c, instr| apply_instruction(&c, instr));

    println!("ended up at ({:#?})", c);
    Ok(())
}

#[derive(std::clone::Clone, Copy, Debug, PartialEq)]
struct Coords(i32, i32, i32, i32);

fn apply_instruction(c: &Coords, instr: Instruction) -> Coords {
    match instr {
        Instruction::Direction(dir, qty) => move_waypoint(c, &dir, qty),
        Instruction::Rotation(rot) => rotate_waypoint(c, &rot),
        Instruction::Forward(times) => move_towards_waypoint(c, times),
    }
}

fn move_waypoint(c: &Coords, direction: &Direction, qty: i32) -> Coords {
    match direction {
        Direction::N => Coords(c.0, c.1 + qty, c.2, c.3),
        Direction::E => Coords(c.0 + qty, c.1, c.2, c.3),
        Direction::S => Coords(c.0, c.1 - qty, c.2, c.3),
        Direction::W => Coords(c.0 - qty, c.1, c.2, c.3),
    }
}

fn move_towards_waypoint(c: &Coords, times: i32) -> Coords {
    (1..=times).fold(*c, |c, _y| Coords(c.0, c.1, c.2 + c.0, c.3 + c.1))
}

fn rotate_waypoint(c: &Coords, rot: &Rotation) -> Coords {
    match rot {
        Rotation::R(90) | Rotation::L(270) => Coords(c.1, -c.0, c.2, c.3),
        Rotation::L(90) | Rotation::R(270) => Coords(-c.1, c.0, c.2, c.3),
        Rotation::L(180) | Rotation::R(180) => Coords(-c.0, -c.1, c.2, c.3),
        _ => panic!("{:?} is not handled", rot),
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

#[derive(std::clone::Clone, Copy, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_towards_waypoint() {
        assert_eq!(
            move_towards_waypoint(&Coords(10, 1, 0, 0), 10),
            Coords(10, 1, 100, 10),
        );
    }

    #[test]
    fn test_rotation() {
        assert_eq!(
            rotate_waypoint(&Coords(10, 4, 0, 0), &Rotation::R(90)),
            Coords(4, -10, 0, 0),
        );
        assert_eq!(
            rotate_waypoint(&Coords(10, 4, 0, 0), &Rotation::L(90)),
            Coords(-4, 10, 0, 0),
        );
    }

    #[test]
    fn test_apply_instruction() {
        let c = apply_instruction(&Coords(10, 1, 0, 0), Instruction::Forward(10));
        let c = apply_instruction(&c, Instruction::Direction(Direction::N, 3));
        let c = apply_instruction(&c, Instruction::Forward(7));
        let c = apply_instruction(&c, Instruction::Rotation(Rotation::R(90)));
        let c = apply_instruction(&c, Instruction::Forward(11));
        assert_eq!(c, Coords(4, -10, 214, -72));
    }
}

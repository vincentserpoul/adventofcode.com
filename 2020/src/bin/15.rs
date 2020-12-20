use std::collections::HashMap;

type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> ResultMain<()> {
    let mut t = Turns::new(vec![2, 1, 10, 11, 0, 6]);
    t.next_til_turn(2020);

    println!("{}", t.turns.last().unwrap());

    t.next_til_turn(30000000);

    println!("{}", t.turns.last().unwrap());

    Ok(())
}

#[derive(std::clone::Clone, Debug, PartialEq)]
struct Turns {
    turns: Vec<usize>,
    mem: HashMap<usize, Vec<usize>>,
}

impl Turns {
    fn new(i: Vec<usize>) -> Turns {
        Turns {
            turns: i.clone(),
            mem: i.iter().enumerate().fold(HashMap::new(), |mut hm, v| {
                hm.entry(*v.1).or_insert(Vec::new()).push(v.0 + 1);
                hm
            }),
        }
    }

    fn next(&mut self) {
        let curr = *self.turns.last().unwrap();
        let next: usize = match self.mem[&curr].len() {
            0 | 1 => 0,
            _ => {
                self.mem[&curr][self.mem[&curr].len() - 1]
                    - self.mem[&curr][self.mem[&curr].len() - 2]
            }
        };

        self.turns.push(next);
        self.mem
            .entry(next)
            .or_insert(Vec::new())
            .push(self.turns.len());
    }

    fn next_til_turn(&mut self, t: usize) -> usize {
        while self.turns.len() < t {
            self.next();
        }

        *self.turns.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turns_init() {
        assert_eq!(
            Turns::new(vec![0, 3, 6]),
            Turns {
                turns: vec![0, 3, 6],
                mem: [(0, vec![1]), (3, vec![2]), (6, vec![3])]
                    .iter()
                    .cloned()
                    .collect(),
            }
        )
    }

    #[test]
    fn test_turns_next_til() {
        let mut t = Turns::new(vec![0, 3, 6]);
        t.next_til_turn(5);
        assert_eq!(
            t,
            Turns {
                turns: vec![0, 3, 6, 0, 3],
                mem: [(0, vec![1, 4]), (3, vec![2, 5]), (6, vec![3])]
                    .iter()
                    .cloned()
                    .collect(),
            }
        )
    }

    #[test]
    fn test_turns_next_til_2020() {
        let mut t = Turns::new(vec![0, 3, 6]);
        t.next_til_turn(2020);
        assert_eq!(*t.turns.last().unwrap(), 436);

        let mut t = Turns::new(vec![1, 3, 2]);
        t.next_til_turn(2020);
        assert_eq!(*t.turns.last().unwrap(), 1);

        let mut t = Turns::new(vec![0, 3, 6]);
        t.next_til_turn(30000000);
        assert_eq!(*t.turns.last().unwrap(), 175594);
    }
}

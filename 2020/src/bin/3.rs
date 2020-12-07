type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let forest = include_str!("../../inputs/3.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|line| line.chars().map(Square::from).collect::<Vec<Square>>())
        .collect::<Vec<Vec<Square>>>();

    let res: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|slope: &(usize, usize)| {
            let mut travel = Travel {
                curr_coords: (0, 0),
                curr_trees: 0,
            };

            let mut is_end = false;
            while !is_end {
                is_end = travel.walk(&forest, slope.0, slope.1);
            }

            println!("{:}", travel.curr_trees);
            travel.curr_trees
        })
        .fold(1, |mut acc: usize, curr_trees: usize| {
            acc *= curr_trees;
            acc
        });

    println!("{:}", res);

    Ok(())
}

#[derive(Debug, std::cmp::PartialEq)]
enum Square {
    Tree,
    Open,
}

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            '#' => Square::Tree,
            '.' => Square::Open,
            _ => panic!("it's not cool"),
        }
    }
}

struct Travel {
    curr_coords: (usize, usize),
    curr_trees: usize,
}

impl Travel {
    fn walk(&mut self, forest: &Vec<Vec<Square>>, x: usize, y: usize) -> bool {
        if forest.is_empty() {
            panic!("forest is too small");
        }

        let forest_max_y = forest.len() - 1;
        let forest_max_x = forest[0].len() - 1;

        self.curr_coords.0 = (self.curr_coords.0 + x) % (forest_max_x + 1);
        self.curr_coords.1 += y;

        if forest[self.curr_coords.1][self.curr_coords.0] == Square::Tree {
            self.curr_trees += 1;
        }

        if self.curr_coords.1 == forest_max_y {
            return true;
        }

        false
    }
}

// cargo run --release --bin 1.2

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let count_valid_passes = include_str!("../../inputs/2.txt")
        .split('\n')
        .filter(|s| {
            let v = s.split(": ").collect::<Vec<&str>>();
            if v.len() != 2 {
                return false;
            }
            let rule: Rule = Rule::from(v[0]);
            rule.is_valid(v[1])
        })
        .count();

    println!("{:}", count_valid_passes);

    Ok(())
}

#[derive(Debug)]
struct Rule {
    indices: (usize, usize),
    c: char,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let (indices, c): (&str, char) = match s.split(' ').collect::<Vec<&str>>() {
            x if x.len() == 2 && x[1].chars().count() == 1 => (x[0], x[1].chars().next().unwrap()),
            _ => ("0", '0'),
        };

        let (index_0, index_1) = match indices.split('-').collect::<Vec<&str>>() {
            x if x.len() == 2 => (x[0], x[1]),
            _ => ("1", "1"),
        };

        let i0 = index_0.parse::<usize>().unwrap();
        let i1 = index_1.parse::<usize>().unwrap();

        Rule {
            indices: (i0, i1),
            c,
        }
    }
}

impl Rule {
    fn is_valid(&self, password: &str) -> bool {
        let c1 = password.chars().nth(&self.indices.0 - 1).unwrap();
        let c2 = password.chars().nth(&self.indices.1 - 1).unwrap();

        (c1 == self.c && c2 != self.c) || (c1 != self.c && c2 == self.c)
    }
}

// cargo run --release --bin 1.2

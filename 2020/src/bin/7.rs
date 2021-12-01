type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> ResultMain<()> {
    let all_bags = include_str!("../../inputs/7.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|bag_contains| BagStatement::from_str(bag_contains).unwrap())
        .collect::<BagMap>();

    println!("{:#?}", count_shiny_gold(&all_bags));

    println!("{:#?}", sum_bags(&all_bags, "shiny gold") - 1);
    Ok(())
}

use std::collections::HashMap;

type BagMap = HashMap<String, Vec<(u32, String)>>;

use std::iter::FromIterator;

impl FromIterator<BagStatement> for BagMap {
    fn from_iter<I: IntoIterator<Item = BagStatement>>(iter: I) -> Self {
        let mut c = BagMap::new();

        for i in iter {
            c.insert(i.container, i.contains);
        }

        c
    }
}

fn count_shiny_gold(bag_map: &BagMap) -> usize {
    bag_map
        .iter()
        .filter(|(_b, bag_contains)| can_contain_shiny_gold(bag_map, *bag_contains))
        .count()
}

fn can_contain_shiny_gold(bag_map: &BagMap, bag_contains: &[(u32, String)]) -> bool {
    bag_contains.iter().any(|(_count, bag)| bag == "shiny gold")
        || bag_contains
            .iter()
            .any(|(_count, bag)| can_contain_shiny_gold(bag_map, bag_map.get(bag).unwrap()))
}

fn sum_bags(bag_map: &BagMap, bag_type: &str) -> u32 {
    1 + bag_map
        .get(bag_type)
        .unwrap()
        .iter()
        .map(|(c, b)| c * sum_bags(bag_map, b))
        .sum::<u32>()
}

#[derive(Debug, PartialEq)]
struct BagStatement {
    container: String,
    contains: Vec<(u32, String)>,
}

use std::str::FromStr;

impl FromStr for BagStatement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sl = String::from(s);
        let mut b = sl[..sl.len() - 1].split(" contain ");
        let container = b
            .next()
            .unwrap()
            .split_whitespace()
            .filter(|w| *w != "bags")
            .collect::<Vec<&str>>()
            .join(" ");
        let contains = b
            .next()
            .unwrap()
            .split(", ")
            .filter(|s| *s != "no other bags")
            .map(|bc| {
                let mut bc_q = bc.split_whitespace();
                (
                    bc_q.next().unwrap().parse::<u32>().unwrap(),
                    bc_q.filter(|w| *w != "bags" && *w != "bag")
                        .collect::<Vec<&str>>()
                        .join(" "),
                )
            })
            .collect::<Vec<(u32, String)>>();

        Ok(BagStatement {
            container,
            contains,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag_statement_from() {
        assert_eq!(
            BagStatement::from_str("dark magenta bags contain 4 striped purple bags, 5 bright lime bags, 2 dull magenta bags, 2 dim black bags.").unwrap(),
            BagStatement {
                container: String::from("dark magenta"),
                contains: vec![(4, String::from("striped purple")), (5, String::from("bright lime")), (2, String::from("dull magenta")), (2, String::from("dim black"))],
            }
        );
    }
}

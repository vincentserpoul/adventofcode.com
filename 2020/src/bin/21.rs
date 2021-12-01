use std::collections::{HashMap, HashSet};

type ResultMain<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> ResultMain<()> {
    let mut food_list = include_str!("../../inputs/21.txt")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|food| Food::from_str(food).unwrap());

    let all_ingredients = get_all_ingredients(&mut food_list.clone());

    let allergens = get_allergens(&mut food_list.clone());

    let non_allergenic_ingredients = get_non_allergenic_ingredients(&allergens, &all_ingredients);

    println!(
        "{}",
        count_appearance(&mut food_list, non_allergenic_ingredients)
    );

    let dangerous_ingredients = get_dangerous_ingredients(&allergens);
    println!("{:?}", dangerous_ingredients.join(","));

    Ok(())
}

#[derive(std::clone::Clone, Debug, PartialEq)]
struct Food(HashSet<String>, HashSet<String>);

use std::str::FromStr;

impl FromStr for Food {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_s = s.replace("(", "").replace(")", "");
        let all = clean_s.split(" contains ").collect::<Vec<&str>>();
        if all.len() != 2 {
            return Err(());
        }
        Ok(Food(
            all[0].split_whitespace().map(|e| e.to_owned()).collect(),
            all[1].split(", ").map(|e| e.to_owned()).collect(),
        ))
    }
}

fn get_all_ingredients(food_list: &mut dyn Iterator<Item = Food>) -> HashSet<String> {
    food_list
        .map(|f| f.0)
        .flatten()
        .collect::<HashSet<String>>()
}

fn get_allergens(food_list: &mut dyn Iterator<Item = Food>) -> HashMap<String, HashSet<String>> {
    food_list
        .flat_map(|food| {
            food.1
                .clone()
                .iter()
                .map(|allergen| (allergen.clone(), food.0.clone().into_iter().collect()))
                .collect::<Vec<(String, HashSet<String>)>>()
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, HashSet<String>>, allergen| {
                let e = acc.entry(allergen.0).or_insert(allergen.1.clone());
                *e = allergen.1.intersection(e).map(|e| e.to_owned()).collect();
                acc
            },
        )
}

fn get_non_allergenic_ingredients(
    allergens: &HashMap<String, HashSet<String>>,
    all_ingredients: &HashSet<String>,
) -> HashSet<String> {
    let all_allergenic_ingredients: HashSet<String> = allergens
        .iter()
        .map(|(_al, ingredients)| ingredients)
        .flatten()
        .map(|e| e.to_owned())
        .collect();

    all_ingredients
        .difference(&all_allergenic_ingredients)
        .map(|e| e.to_owned())
        .collect()
}

fn count_appearance(
    food_list: &mut dyn Iterator<Item = Food>,
    non_allergenic_ingredients: HashSet<String>,
) -> usize {
    food_list
        .map(|f| f.0.intersection(&non_allergenic_ingredients).count())
        .sum()
}

fn get_dangerous_ingredients(allergens: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let dangerous_ingredients_set = allergens
        .iter()
        .flat_map(|(a, ingrs)| {
            ingrs
                .iter()
                .map(|i| (i.to_string(), a.to_string()))
                .collect::<Vec<(String, String)>>()
        })
        .collect::<HashSet<(String, String)>>();

    dbg!(&dangerous_ingredients_set);

    let mut dangerous_ingredients = dangerous_ingredients_set
        .into_iter()
        .collect::<Vec<(String, String)>>();

    dangerous_ingredients.sort_by(|a, b| a.1.cmp(&b.1));

    dangerous_ingredients
        .iter()
        .map(|(i, _a)| i.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turns_init() {
        assert_eq!(
            Food::from_str("mxmxvkd kfcds (contains dairy, fish)"),
            Ok(Food(
                ["mxmxvkd".to_string(), "kfcds".to_string()]
                    .iter()
                    .cloned()
                    .collect(),
                vec!["dairy".to_string(), "fish".to_string()]
                    .iter()
                    .cloned()
                    .collect(),
            )),
        )
    }

    #[test]
    fn test_get_non_allergenic_ingredients() {
        let food = vec![
            Food::from_str("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)").unwrap(),
            Food::from_str("trh fvjkl sbzzf mxmxvkd (contains dairy)").unwrap(),
            Food::from_str("sqjhc fvjkl (contains soy)").unwrap(),
            Food::from_str("sqjhc mxmxvkd sbzzf (contains fish)").unwrap(),
        ];
        let mut food_list = food.into_iter();

        let all_ingredients = get_all_ingredients(&mut food_list.clone());
        let allergens = get_allergens(&mut food_list);

        assert_eq!(
            get_non_allergenic_ingredients(&allergens, &all_ingredients),
            [
                "kfcds".to_string(),
                "nhms".to_string(),
                "sbzzf".to_string(),
                "trh".to_string()
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn test_count_appearance() {
        let food = vec![
            Food::from_str("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)").unwrap(),
            Food::from_str("trh fvjkl sbzzf mxmxvkd (contains dairy)").unwrap(),
            Food::from_str("sqjhc fvjkl (contains soy)").unwrap(),
            Food::from_str("sqjhc mxmxvkd sbzzf (contains fish)").unwrap(),
        ];
        let mut food_list = food.into_iter();
        let all_ingredients = get_all_ingredients(&mut food_list.clone());
        let allergens = get_allergens(&mut food_list.clone());

        let non_allerg = get_non_allergenic_ingredients(&allergens, &all_ingredients);
        assert_eq!(count_appearance(&mut food_list, non_allerg), 5);
    }

    #[test]
    fn test_get_dangerous_ingredients() {
        let food = vec![
            Food::from_str("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)").unwrap(),
            Food::from_str("trh fvjkl sbzzf mxmxvkd (contains dairy)").unwrap(),
            Food::from_str("sqjhc fvjkl (contains soy)").unwrap(),
            Food::from_str("sqjhc mxmxvkd sbzzf (contains fish)").unwrap(),
        ];
        let mut food_list = food.into_iter();
        let allergens = get_allergens(&mut food_list);

        assert_eq!(
            get_dangerous_ingredients(&allergens),
            vec![
                "fvjkl".to_string(),
                "mxmxvkd".to_string(),
                "sqjhc".to_string(),
            ],
        );
    }
}

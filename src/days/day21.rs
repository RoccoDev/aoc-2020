use fxhash::FxHashMap;
use regex::Regex;

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"(.+) \(contains (.+)\)"#).unwrap();
}

struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn calc_allergens(foods: &[Food]) -> Vec<(String, String)> {
    let mut allergens = FxHashMap::default();
    for food in foods {
        for allergen in &food.allergens {
            allergens
                .entry(allergen)
                .or_insert(vec![])
                .push(&food.ingredients);
        }
    }
    let mut res = vec![];
    for (allergen, ingredients) in allergens {
        let mut to_assign: Vec<_> = ingredients
            .iter()
            .map(|v| v.iter())
            .flatten()
            .filter(|i| ingredients.iter().all(|v| v.contains(i)))
            .collect();
        to_assign.sort();
        to_assign.dedup();
        res.push((allergen.to_string(), to_assign));
    }
    // Day 16 flashbacks
    crate::reduce_possibilities(&mut res);
    res.into_iter()
        .map(|(k, v)| (k, v.first().map(|s| s.to_string()).unwrap()))
        .collect()
}

fn parse_input(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|l| {
            let caps = REGEX.captures(l).unwrap();
            Food {
                ingredients: caps[1].split(' ').map(|s| s.to_string()).collect(),
                allergens: caps[2].split(", ").map(|s| s.to_string()).collect(),
            }
        })
        .collect()
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    let foods = parse_input(input);
    // The ingredients that are known to contain allergens
    let ingredients: Vec<String> = calc_allergens(&foods).into_iter().map(|(_, i)| i).collect();
    foods
        .iter()
        .map(|f| {
            f.ingredients
                .iter()
                .filter(|i| !ingredients.contains(i))
                .count()
        })
        .sum()
}

#[aoc(day21, part2)]
fn part2(input: &str) -> String {
    let foods = parse_input(input);
    // The ingredients that are known to contain allergens (Thank God I designed this with a possible Part 2 in mind!)
    let mut ingredients = calc_allergens(&foods);
    ingredients.sort_by_key(|(allergen, _)| allergen.clone());
    ingredients
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .fold(String::new(), |a, b| a + "," + &b)
        .strip_prefix(',')
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn part2_example() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(&part2(input), "mxmxvkd,sqjhc,fvjkl");
    }
}

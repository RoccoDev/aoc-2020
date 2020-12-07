use fxhash::FxHashMap;
use regex::Regex;

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"(.+) bags contain (?:(no other bags)|(.+))\."#).unwrap();
    static ref SINGLE: Regex = Regex::new(r#"\s?(\d) (.+) bags?"#).unwrap();
}

#[derive(Default)]
struct Bag {
    children: Vec<BagChild>,
}

struct BagChild {
    qty: usize,
    name: String,
}

impl Bag {
    fn parse(line: &str) -> (String, Bag) {
        let caps = REGEX.captures(line).expect("line didn't match pattern");
        let name = caps[1].to_string();
        if let Some(_) = caps.get(2) {
            return (name, Bag::default());
        }
        let children = caps[3]
            .split(',')
            .map(|part| {
                let caps = SINGLE.captures(part).expect("part didn't match pattern");
                BagChild {
                    qty: caps[1].parse().unwrap(),
                    name: caps[2].to_string(),
                }
            })
            .collect();
        (name, Bag { children })
    }
}

fn parse_input(input: &str) -> FxHashMap<String, Bag> {
    input.lines().map(|l| Bag::parse(l)).collect()
}

fn has_gold(map: &FxHashMap<String, Bag>, bag: &Bag) -> bool {
    for child in &bag.children {
        if child.name == "shiny gold" {
            return true;
        }
        let bag = map.get(&child.name);
        if let Some(bag) = bag {
            if has_gold(&map, &bag) {
                return true;
            }
        }
    }
    false
}

fn deep_count(map: &FxHashMap<String, Bag>, bag: &Bag, count: &mut usize) {
    for child in &bag.children {
        *count += child.qty;
        let bag = map.get(&child.name);
        if let Some(bag) = bag {
            let mut tmp = 0;
            deep_count(&map, &bag, &mut tmp);
            tmp *= child.qty;
            *count += tmp;
        }
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let bags = parse_input(input);
    bags.values().filter(|b| has_gold(&bags, b)).count()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let bags = parse_input(input);
    let mut count = 0;
    deep_count(&bags, &bags["shiny gold"], &mut count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn part2_example() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let part1 = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(part2(input), 126);
        assert_eq!(part2(part1), 32);
    }
}

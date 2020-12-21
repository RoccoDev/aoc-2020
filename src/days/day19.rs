use fxhash::FxHashMap;
use regex::Regex;

lazy_static::lazy_static! {
    static ref RULE: Regex = Regex::new(r#"(\d+): (?:"(.+)"|(.+) \| (.+)|(.+))"#).unwrap();
}

#[derive(Debug, Clone)]
enum Rule {
    Simple(char),
    Multiple(Vec<u8>),
    Either(Vec<Vec<u8>>),
}

struct Matcher {
    rules: FxHashMap<u8, Rule>,
}

impl Matcher {
    fn multiple_match<'a>(&self, message: &'a str, rules: &Vec<u8>) -> Vec<&'a str> {
        let mut messages = vec![message];
        for rule in rules {
            let mut new_msgs = vec![];
            for message in messages {
                new_msgs.append(&mut self.matches(&message, *rule))
            }
            messages = new_msgs;
        }
        messages
    }

    fn matches<'a>(&self, message: &'a str, rule: u8) -> Vec<&'a str> {
        if message.is_empty() {
            return vec![];
        }
        match &self.rules[&rule] {
            Rule::Simple(c) => message
                .strip_prefix(*c)
                .and_then(|m| Some(vec![m]))
                .unwrap_or_else(|| vec![]),
            Rule::Multiple(rules) => self.multiple_match(message, &rules),
            Rule::Either(possible) => possible
                .iter()
                .map(|p| self.multiple_match(message, p))
                .flatten()
                .collect(),
        }
    }
}

fn parse_input(input: &str) -> (Matcher, Vec<&'_ str>) {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap();
    let messages = parts.next().unwrap();
    let mut map = FxHashMap::default();
    for rule in rules.lines() {
        let caps = RULE.captures(rule).unwrap();
        let id = caps[1].parse().unwrap();
        if let Some(simple) = caps.get(2) {
            map.insert(id, Rule::Simple(simple.as_str().chars().next().unwrap()));
        } else if let Some(one) = caps.get(3) {
            map.insert(
                id,
                Rule::Either(vec![
                    one.as_str()
                        .split(' ')
                        .map(|c| c.parse().unwrap())
                        .collect(),
                    caps.get(4)
                        .unwrap()
                        .as_str()
                        .split(' ')
                        .map(|c| c.parse().unwrap())
                        .collect(),
                ]),
            );
        } else if let Some(multiple) = caps.get(5) {
            map.insert(
                id,
                Rule::Multiple(
                    multiple
                        .as_str()
                        .split(' ')
                        .map(|c| c.parse().unwrap())
                        .collect(),
                ),
            );
        }
    }
    (Matcher { rules: map }, messages.lines().collect())
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let (matcher, messages) = parse_input(input);
    messages
        .into_iter()
        .map(|m| {
            matcher
                .matches(m, 0)
                .iter()
                .filter(|v| v.is_empty())
                .count()
        })
        .sum()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    let (mut matcher, messages) = parse_input(input);
    matcher
        .rules
        .insert(8, Rule::Either(vec![vec![42], vec![42, 8]]));
    matcher
        .rules
        .insert(11, Rule::Either(vec![vec![42, 31], vec![42, 11, 31]]));
    messages
        .into_iter()
        .map(|m| {
            matcher
                .matches(m, 0)
                .iter()
                .filter(|v| v.is_empty())
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        assert_eq!(part1(input), 2);
    }
}

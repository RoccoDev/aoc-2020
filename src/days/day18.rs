use itertools::Itertools;
use regex::Regex;

lazy_static::lazy_static! {
    /// Get the input inside parentheses. If they're nested, it gets the innermost one so it can recursively simplify the input.
    static ref PARENTHESES: Regex = Regex::new(r#"\([^()]+\)"#).unwrap();
    static ref INPUT_TRANSFORM: Regex = Regex::new(r#"\d+ \+ \d+"#).unwrap();
}

fn run_operations(input: &str, other_precendence: bool) -> u64 {
    let mut res = String::from(input);
    loop {
        let for_regex = res.clone();
        match PARENTHESES.find(&for_regex) {
            Some(m) => {
                let parsed =
                    run_operations(&res[m.start() + 1..m.end() - 1], other_precendence).to_string();
                res.replace_range(m.range(), &parsed);
            }
            None => {
                if !other_precendence {
                    break;
                } else {
                    // Evaluate additions first (same method as parentheses), but only when everything else has been fully simplified.
                    match INPUT_TRANSFORM.find(&for_regex) {
                        Some(m) => {
                            // Run the additions ignoring the new precedence rules, it's just additions so no need to check
                            let parsed = run_operations(&res[m.range()], false).to_string();
                            res.replace_range(m.range(), &parsed);
                        }
                        None => break,
                    }
                }
            }
        }
    }
    // Can't further simplify, run the operations
    let mut iter = res.split(' ');
    let mut result = iter.next().unwrap().parse().unwrap();
    for (op, value) in iter.tuples() {
        match (op, value) {
            ("*", v) => result *= v.parse::<u64>().unwrap(),
            ("+", v) => result += v.parse::<u64>().unwrap(),
            (o, v) => panic!("Invalid pair {} {}", o, v),
        }
    }
    result
}

#[aoc(day18, part1)]
fn part1(input: &str) -> u64 {
    input.lines().map(|l| run_operations(l, false)).sum()
}

#[aoc(day18, part2)]
fn part2(input: &str) -> u64 {
    input.lines().map(|l| run_operations(l, true)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(
            part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}

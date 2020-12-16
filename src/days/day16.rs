use std::ops::RangeInclusive;

use fxhash::FxHashMap;
use regex::Regex;

struct Input {
    fields: FxHashMap<String, (RangeInclusive<u64>, RangeInclusive<u64>)>,
    own_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

lazy_static::lazy_static! {
    static ref FIELD_VALIDITY: Regex = Regex::new(r#"(.+): (\d+)-(\d+) or (\d+)-(\d+)"#).unwrap();
}

fn parse_input(input: &str) -> Input {
    let mut fields = FxHashMap::default();
    let mut nearby_tickets = vec![];
    let parts: Vec<&str> = input.split("\n\n").collect();
    for line in parts[0].lines() {
        if let Some(caps) = FIELD_VALIDITY.captures(line) {
            let name = String::from(&caps[1]);
            let ranges = (
                caps[2].parse().unwrap()..=caps[3].parse().unwrap(),
                caps[4].parse().unwrap()..=caps[5].parse().unwrap(),
            );
            fields.insert(name, ranges);
        }
    }
    let own_ticket = parts[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();
    for line in parts[2].lines().skip(1) {
        nearby_tickets.push(line.split(',').map(|l| l.parse().unwrap()).collect());
    }
    Input {
        fields,
        own_ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
fn part1(input: &str) -> u64 {
    let input = parse_input(input);
    input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|i| {
            input
                .fields
                .values()
                .all(|range| !range.0.contains(i) && !range.1.contains(i))
        })
        .sum()
}

#[aoc(day16, part2)]
fn part2(input: &str) -> u64 {
    let input = parse_input(input);
    let mut valid: Vec<_> = input
        .nearby_tickets
        .iter()
        .filter(|v| {
            v.iter().all(|i| {
                input
                    .fields
                    .values()
                    .any(|ranges| ranges.0.contains(&i) || ranges.1.contains(&i))
            })
        })
        .collect();
    valid.push(&input.own_ticket);
    let mut res = vec![];
    for (s, ranges) in input.fields.iter() {
        let mut possibilities = vec![];
        for idx in 0..input.fields.len() {
            if valid
                .iter()
                .map(|v| v[idx])
                .all(|i| ranges.0.contains(&i) || ranges.1.contains(&i))
            {
                possibilities.push(idx);
            }
        }
        res.push((s, possibilities));
    }
    let old_snap = res.clone();
    for (idx, possibilities) in res.iter_mut().enumerate() {
        *possibilities = (
            possibilities.0,
            possibilities
                .1
                .iter()
                .filter(|i| {
                    !old_snap
                        .iter()
                        .enumerate()
                        .filter_map(|(idx2, x)| if idx2 != idx { Some(x) } else { None })
                        .all(|v| v.1.contains(i))
                })
                .map(|&v| v)
                .collect::<Vec<usize>>(),
        );
    }
    // Reduce possibilities to one each
    loop {
        let snapshot = res.clone();
        let mut changed = 0;
        for (i, possibilities) in snapshot.iter().enumerate() {
            match possibilities.1.len() {
                0 => {
                    res[i] = old_snap[i].clone();
                }
                1 => {
                    let x = possibilities.1[0];
                    for (i1, poss2) in res.iter_mut().enumerate() {
                        if i != i1 {
                            let old_len = poss2.1.len();
                            *poss2 = (
                                poss2.0,
                                poss2
                                    .1
                                    .iter()
                                    .filter_map(|y| if *y != x { Some(*y) } else { None })
                                    .collect::<Vec<_>>(),
                            );
                            if poss2.1.len() != old_len {
                                changed += 1;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        if changed == 0 {
            break;
        }
    }
    res.into_iter()
        .filter_map(|v| {
            if v.0.starts_with("departure") {
                Some(input.own_ticket[v.1[0]])
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(part1(input), 71);
    }
}

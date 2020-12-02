use regex::Regex;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    letter: char,
    input: Vec<char>,
}

impl Password {
    fn valid_part1(&self) -> bool {
        (self.min..=self.max).contains(&self.input.iter().filter(|c| **c == self.letter).count())
    }

    fn valid_part2(&self) -> bool {
        (self.input[self.min - 1] == self.letter) ^ (self.input[self.max - 1] == self.letter)
    }
}

fn parse_input(input: &str) -> Vec<Password> {
    let regex = Regex::new("(\\d+)-(\\d+) ([a-z]): ([a-z]+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = regex.captures(line).unwrap();
            Password {
                min: caps[1].parse().unwrap(),
                max: caps[2].parse().unwrap(),
                letter: caps[3].chars().next().unwrap(),
                input: caps[4].chars().collect(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|password| password.valid_part1())
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|password| password.valid_part2())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        assert_eq!(part1("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 1);
    }
}

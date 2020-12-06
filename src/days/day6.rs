struct Group {
    answers: Vec<char>,
    members: usize,
}

impl Group {
    fn parse(input: &str) -> Group {
        let people: Vec<&str> = input.lines().collect();
        let answers = {
            let mut answers: Vec<char> = people.iter().map(|l| l.chars()).flatten().collect();
            answers.sort();
            answers
        };
        Group {
            answers,
            members: people.len(),
        }
    }

    fn answers(&mut self) -> usize {
        self.answers.dedup();
        self.answers.len()
    }

    fn consensus(&self) -> usize {
        // Operate on the deduplicated version (we want to filter the unique answers)
        let mut dedup = self.answers.clone();
        dedup.dedup();
        dedup
            .iter()
            // Count the occurrences in the original version, and check if they're the same as the member count
            .filter(|a| self.answers.iter().filter(|c| c == a).count() == self.members)
            .count()
    }
}

fn parse_input(input: &str) -> Vec<Group> {
    input.split("\n\n").map(|g| Group::parse(g)).collect()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|mut g| g.answers())
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    parse_input(input).iter().map(|g| g.consensus()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn part2_example() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(part2(input), 6);
    }
}

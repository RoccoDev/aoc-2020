struct Circle {
    neighbours: Vec<usize>,
    current: usize,
    len: usize,
}

impl Circle {
    fn parse(input: &str, extend_to: usize) -> Circle {
        let src: Vec<usize> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let mut neighbours = Vec::with_capacity(extend_to + 1);
        for x in 0..extend_to + 1 {
            neighbours.push(x + 1);
        }
        for idx in 0..src.len() - 1 {
            let first = src[idx];
            let next = src[idx + 1];
            neighbours[first] = next;
        }
        neighbours[src[src.len() - 1]] = if src.len() == extend_to {
            src[0]
        } else {
            src.len() + 1
        };
        if neighbours.len() > src.len() + 1 {
            let idx = neighbours.len() - 1;
            neighbours[idx] = src[0];
        }
        Circle {
            neighbours,
            current: src[0],
            len: extend_to,
        }
    }

    fn step(&mut self) {
        let current = self.current;
        let first_pick = self.neighbours[current];
        let second_pick = self.neighbours[first_pick];
        let third_pick = self.neighbours[second_pick];
        let picked = [first_pick, second_pick, third_pick];
        let after_third = self.neighbours[third_pick];
        self.neighbours[current] = after_third;
        self.current = after_third;
        let mut dest_idx = current - 1;
        loop {
            if dest_idx == 0 {
                dest_idx = self.len;
            } else if picked.contains(&dest_idx) {
                dest_idx -= 1;
            } else {
                break;
            }
        }
        let dest = self.neighbours[dest_idx];
        self.neighbours[dest_idx] = first_pick;
        self.neighbours[third_pick] = dest;
    }

    fn display(&self) -> String {
        let mut buf = String::with_capacity(self.len);
        let mut tmp = self.neighbours[1];
        while tmp != 1 {
            buf += &tmp.to_string();
            tmp = self.neighbours[tmp];
        }
        buf
    }
}

#[aoc(day23, part1)]
fn part1(input: &str) -> String {
    let mut circle = Circle::parse(input, 9);
    for _ in 0..100 {
        circle.step();
    }
    circle.display()
}

#[aoc(day23, part2)]
fn part2(input: &str) -> u64 {
    let mut circle = Circle::parse(input, 1e6 as usize);
    for _ in 0..(1e7 as usize) {
        circle.step()
    }
    circle.neighbours[1] as u64 * circle.neighbours[circle.neighbours[1]] as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(&part1("389125467"), "67384529");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("389125467"), 149245887792);
    }
}

struct Combinations {
    /// The numbers >= 1000
    high: Vec<i32>,
    /// The other numbers
    low: Vec<i32>,
}

fn parse_numbers(input: &str) -> Combinations {
    let mut high = vec![];
    let mut low = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let num = line.parse().unwrap();
        (if num >= 1000 { &mut high } else { &mut low }).push(num);
    }
    Combinations { high, low }
}

fn parse_all(input: &str) -> Vec<i32> {
    let mut vec = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let num = line.parse().unwrap();
        vec.push(num);
    }
    vec
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let combos = parse_numbers(input);
    for high in &combos.high {
        for low in &combos.low {
            if low + high == 2020 {
                return low * high;
            }
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let nums = {
        let mut nums = parse_all(input);
        nums.sort();
        nums
    };
    for num1 in &nums {
        for num2 in &nums {
            let sum = 2020 - (num1 + num2);
            if nums.binary_search(&sum).is_ok() {
                // https://www.wolframalpha.com/input/?i=xyz%3B+x%2By%2Bz%3D2020
                return -num1 * num2 * (num1 + num2 - 2020);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = example!(1721 979 366 299 675 1456);
        assert_eq!(part1(input), 514579);
    }

    #[test]
    fn part2_example() {
        let input = example!(1721 979 366 299 675 1456);
        println!("{}", input);
        assert_eq!(part2(input), 241861950);
    }
}

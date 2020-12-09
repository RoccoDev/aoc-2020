use itertools::Itertools;

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn calc_part1(nums: &[i64], preamble_len: usize) -> i64 {
    for i in preamble_len..nums.len() {
        let num = nums[i];
        let slice = &nums[(i - preamble_len)..i];
        if !slice.iter().any(|i| {
            slice
                .iter()
                .find(|i2| **i2 == num - i)
                .and_then(|i2| if i2 == i { None } else { Some(i2) })
                .is_some()
        }) {
            return num;
        }
    }
    0
}

#[aoc(day9, part1)]
fn part1(input: &str) -> i64 {
    let nums = parse_input(input);
    calc_part1(&nums, 25)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> i64 {
    let mut nums = parse_input(input);
    let part1 = calc_part1(&nums, 25);
    nums.retain(|&n| n < part1);
    for i in 0..nums.len() {
        let mut sum = nums[i];
        let mut known = vec![];
        known.push(nums[i]);
        for j in (i + 1)..nums.len() {
            sum += nums[j];
            known.push(nums[j]);
            if sum == part1 {
                return known
                    .iter()
                    .minmax()
                    .into_option()
                    .map(|(a, b)| a + b)
                    .unwrap();
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
        let input = example!(35 20 15 25 47 40 62 55 65 95 102 117 150 182 127 219 299 277 309 576);
        assert_eq!(calc_part1(&parse_input(input), 5), 127);
    }
}

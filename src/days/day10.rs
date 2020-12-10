fn parse_input(input: &str) -> Vec<u8> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

/// Easy sorted array
fn specialize(input: &[u8]) -> [Option<u8>; std::u8::MAX as usize] {
    let mut arr = [None; std::u8::MAX as usize];
    for i in input {
        arr[*i as usize] = Some(*i);
    }
    arr
}

#[aoc(day10, part1)]
fn part1(input: &str) -> i32 {
    let nums = specialize(&parse_input(input));
    let mut diffs = (0, 0); // 1-diff, 3-diff
    let mut idx = 0; // Index, which is also the current joltage
    loop {
        if let Some(j) = nums.get(idx + 1).map(|&x| x).flatten() {
            diffs.0 += 1;
            idx = j as usize;
        } else if let Some(j) = nums.get(idx + 2).map(|&x| x).flatten() {
            idx = j as usize;
        } else if let Some(j) = nums.get(idx + 3).map(|&x| x).flatten() {
            diffs.1 += 1;
            idx = j as usize;
        } else {
            diffs.1 += 1; // Last one is always a 3-diff
            break;
        }
    }
    diffs.0 * diffs.1
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let (nums, max) = {
        let mut nums = parse_input(input);
        nums.sort();
        let max = *nums.last().unwrap() as usize + 3;
        nums.push(max as u8);
        (nums, max)
    };
    let mut arrangements = vec![0; max + 1];
    arrangements[0] = 1;
    for num in nums {
        let num = num as usize;
        arrangements[num] = arrangements[num.saturating_sub(3)..num].iter().sum();
    }
    *arrangements.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = example!(28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3);
        assert_eq!(part1(input), 22 * 10);
    }

    #[test]
    fn part2_example() {
        let input = example!(16 10 15 5 1 11 7 19 6 12 4);
        assert_eq!(part2(input), 8);
    }
}

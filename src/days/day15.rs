use fxhash::FxHashMap;

fn parse_input(input: &str) -> Vec<u32> {
    input.split(',').map(|p| p.parse().unwrap()).collect()
}

fn calc(starting_nums: &[u32], end: u32) -> u32 {
    let mut map: FxHashMap<u32, Vec<u32>> = FxHashMap::default(); // 40% faster than BTreeMap
    let mut turn = 0;
    let mut last = 0;
    // Add starting numbers
    for (i, num) in starting_nums.iter().enumerate() {
        turn += 1;
        last = *num;
        if i < starting_nums.len() {
            map.insert(*num, vec![turn]);
        }
    }
    // Run main loop
    loop {
        let turns = map.entry(last).or_insert(vec![]);
        turns.push(turn);
        if turns.len() == 1 {
            last = 0;
        } else {
            let last_spoken = &turns[(turns.len() - 2)..=(turns.len() - 1)];
            last = last_spoken[1] - last_spoken[0];
            turns.drain(0..turns.len() - 2); // Keep vectors small for long iterations
        }
        turn += 1;
        if turn == end {
            return last;
        }
    }
}

#[aoc(day15, part1)]
fn part1(input: &str) -> u32 {
    calc(&parse_input(input), 2020)
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u32 {
    calc(&parse_input(input), 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("0,3,6"), 436);
        assert_eq!(part1("1,3,2"), 1);
    }
}

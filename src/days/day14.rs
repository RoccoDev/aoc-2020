use fxhash::FxHashMap;
use regex::Regex;

lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"m(?:ask = ([01X]+)|em\[(\d+)\] = (\d+))"#).unwrap();
}

enum Instruction<'a> {
    Mask(&'a str),
    Set(u64, u64),
}

fn parse_input(input: &str) -> Vec<Instruction<'_>> {
    input
        .lines()
        .map(|l| {
            let caps = REGEX.captures(l).expect("input doesn't match pattern");
            match &caps.get(1) {
                Some(mask) => Instruction::Mask(mask.as_str()),
                None => Instruction::Set(caps[2].parse().unwrap(), caps[3].parse().unwrap()),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    // We can represent the "36-bit address space" as an u64 and just ignore the 28 most significant bits.
    // When we use left-shift we have to invert the bit index to avoid adding zeros on the back instead.
    let mut space: FxHashMap<u64, u64> = FxHashMap::default();
    let mut mask = "";
    let instructions = parse_input(input);
    for inst in instructions {
        match inst {
            Instruction::Set(k, mut v) => {
                mask.chars()
                    .enumerate()
                    .filter(|c| c.1 != 'X')
                    .for_each(|(idx, c)| {
                        v ^= (-c.to_digit(10).map(|x| x as i64).unwrap() as u64 ^ v)
                            & (1 << (35 - idx));
                    });
                space.insert(k, v);
            }
            Instruction::Mask(m) => mask = m,
        }
    }
    space.values().sum()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    // We can represent the "36-bit address space" as an u64 and just ignore the 28 most significant bits.
    // When we use left-shift we have to invert the bit index to avoid adding zeros on the back instead.
    let mut space: FxHashMap<String, u64> = FxHashMap::default();
    let mut mask = "";
    let instructions = parse_input(input);
    for inst in instructions {
        match inst {
            Instruction::Set(k, v) => {
                let repr = format!("{:036b}", k);
                let xs: Vec<usize> = mask
                    .chars()
                    .enumerate()
                    .filter(|c| c.1 == 'X')
                    .map(|t| t.0)
                    .collect();
                // Neither idiomatic, nor fast - but creative :)
                for i in 0..2i32.pow(xs.len() as u32) {
                    let x = format!("{:0n$b}", i, n = xs.len());
                    let mut addr = String::with_capacity(repr.len());
                    let mut x_idx = 0;
                    for (i, c) in mask.chars().enumerate() {
                        if xs.contains(&i) {
                            addr.push(x.chars().nth(x_idx).unwrap());
                            x_idx += 1;
                        } else if c == '0' {
                            addr.push(repr.chars().nth(i).unwrap());
                        } else {
                            addr.push(c);
                        }
                    }
                    space.insert(addr, v);
                }
            }
            Instruction::Mask(m) => mask = m,
        }
    }
    space.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(part1(input), 165);
    }

    #[test]
    fn part2_example() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(part2(input), 208);
    }
}

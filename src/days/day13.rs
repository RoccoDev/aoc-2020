use std::cmp::Ordering;

fn parse_input(input: &str) -> (i64, Vec<Option<i64>>) {
    let mut lines = input.lines();
    (
        lines.next().map(|l| l.parse().unwrap()).unwrap(),
        lines
            .next()
            .unwrap()
            .split(',')
            .map(|part| {
                if part == "x" {
                    None
                } else {
                    Some(part.parse().unwrap())
                }
            })
            .collect(),
    )
}

#[aoc(day13, part1)]
fn part1(input: &str) -> i64 {
    let (timestamp, times) = parse_input(input);
    let res = times
        .into_iter()
        .flatten()
        .map(|i| (i, timestamp + i - (timestamp % i)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    res.0 * (res.1 - timestamp)
}

#[aoc(day13, part2)]
fn part2(input: &str) -> i64 {
    let (_, times) = parse_input(input);
    times
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|x| (x - (i as i64 + x - 1) % x - 1, x)))
        .fold((0, 1), |(r1, q1), (r2, q2)| {
            chinese_remainder(r1, q1, r2, q2)
        })
        .0
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut x, mut y) = if a < b { (a, b) } else { (b, a) };
    while x != 0 {
        let tmp = x;
        x = y % x;
        y = tmp;
    }
    y
}

fn chinese_remainder(r1: i64, q1: i64, r2: i64, q2: i64) -> (i64, i64) {
    let mut a = r1;
    let mut b = r2;
    let q = q1 * q2 / gcd(q1, q2);
    loop {
        match a.cmp(&b) {
            Ordering::Less => a += ((b - a + q1 - 1) / q1) * q1,
            Ordering::Equal => return (a, q),
            Ordering::Greater => b += ((a - b + q2 - 1) / q2) * q2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "939
7,13,x,x,59,x,31,19";
        assert_eq!(part1(input), 295);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("0\n17,x,13,19"), 3417);
        assert_eq!(part2("0\n67,7,59,61"), 754018);
        assert_eq!(part2("0\n67,x,7,59,61"), 779210);
        assert_eq!(part2("0\n67,7,x,59,61"), 1261476);
    }
}

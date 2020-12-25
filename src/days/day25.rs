fn reverse_subject(subject: usize, pub_key: usize) -> usize {
    let mut i = 0;
    let mut res = 1;
    while res != pub_key {
        res *= subject;
        res %= 20201227;
        i += 1;
    }
    i
}

fn calc_subject(subject: usize, i: usize) -> usize {
    let mut res = 1;
    for _ in 0..i {
        res *= subject;
        res %= 20201227;
    }
    res
}

fn parse_input(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse().unwrap(),
        lines.next().unwrap().parse().unwrap(),
    )
}

#[aoc(day25, part1)]
fn part1(input: &str) -> usize {
    let (card_pub, door_pub) = parse_input(input);
    let (card_loop, door_loop) = (reverse_subject(7, card_pub), reverse_subject(7, door_pub));
    let (card_enc, door_enc) = (
        calc_subject(door_pub, card_loop),
        calc_subject(card_pub, door_loop),
    );
    assert_eq!(card_enc, door_enc);
    card_enc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = example!(5764801 17807724);
        assert_eq!(part1(input), 14897079);
    }
}

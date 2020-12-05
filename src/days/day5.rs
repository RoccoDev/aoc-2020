struct Seat {
    row: u8,
    column: u8,
}

impl Seat {
    // Reuse the allocation for cols and rows
    fn parse(input: &str, rows: &mut [u8; 128], cols: &mut [u8; 8]) -> Seat {
        for i in 0..=127 {
            rows[i] = i as u8
        }
        for i in 0..=7 {
            cols[i] = i as u8
        }
        let mut rows = (128, &rows[..]);
        let mut cols = (8, &cols[..]);
        for ch in input.chars() {
            match ch {
                'F' => {
                    rows.0 /= 2;
                    rows.1 = &rows.1[0..rows.0];
                }
                'B' => {
                    rows.0 /= 2;
                    rows.1 = &rows.1[rows.0..(rows.0 * 2)];
                }
                'L' => {
                    cols.0 /= 2;
                    cols.1 = &cols.1[0..cols.0];
                }
                'R' => {
                    cols.0 /= 2;
                    cols.1 = &cols.1[cols.0..(cols.0 * 2)];
                }
                c => panic!("Invalid char {}", c),
            }
        }
        Seat {
            row: rows.1[0],
            column: cols.1[0],
        }
    }

    fn get_id(&self) -> i32 {
        self.row as i32 * 8 + self.column as i32
    }
}

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
    let mut rows = [0; 128];
    let mut cols = [0; 8];
    input
        .lines()
        .map(|line| Seat::parse(line, &mut rows, &mut cols).get_id())
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
    let mut rows = [0; 128];
    let mut cols = [0; 8];
    // Sorted vec + binary search is 20% faster than (default) HashSet contains()
    let seats = {
        let mut seats: Vec<i32> = input
            .lines()
            .map(|line| Seat::parse(line, &mut rows, &mut cols).get_id())
            .collect();
        seats.sort();
        seats
    };
    for row in 0..=127 {
        for col in 0..=7 {
            let id = row * 8 + col;
            if !seats.binary_search(&id).is_err()
                && seats.binary_search(&(id + 1)).is_ok()
                && seats.binary_search(&(id - 1)).is_ok()
            {
                return id;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_seat() {
        let mut rows = [0; 128];
        let mut cols = [0; 8];
        assert_eq!(
            Seat::parse("BFFFBBFRRR", &mut rows, &mut cols).get_id(),
            567
        );
    }
}

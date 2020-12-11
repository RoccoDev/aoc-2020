#[derive(Clone)]
struct Grid {
    seats: Vec<Vec<Option<Seat>>>,
    max_x: usize,
}

struct GridIter<'a> {
    grid: &'a mut Grid,
    snapshot: Grid,
    point: (usize, usize),
}

#[derive(Clone, Copy, Debug)]
struct Seat {
    vacancy: SeatVacancy,
}

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum SeatVacancy {
    Empty,
    Occupied,
}

type Item<'a> = (
    &'a mut Option<Seat>,
    &'a Option<Seat>,
    [Option<&'a Seat>; 8],
);
impl<'a> GridIter<'a> {
    fn next(&mut self) -> Option<Item<'_>> {
        let current = self
            .snapshot
            .seats
            .get(self.point.1)
            .and_then(|v| v.get(self.point.0));
        let x = self.point.0;
        let y = self.point.1;
        self.point.0 += 1;
        if self.point.0 >= self.snapshot.max_x {
            self.point.0 = 0;
            self.point.1 += 1;
        }
        if current.is_none() {
            return None;
        }
        let mutable = self.grid.seats.get_mut(y).and_then(|v| v.get_mut(x));
        let mut adjacent = [None; 8];
        let mut i = 0;
        for y1 in -1i32..=1 {
            for x1 in -1i32..=1 {
                if x1 == 0 && y1 == 0 {
                    continue;
                }
                let x = {
                    if x1 < 0 && x == 0 || x1 > 0 && x == self.snapshot.max_x - 1 {
                        None
                    } else {
                        let x = x as i32 + x1;
                        Some(x as usize)
                    }
                };
                if let Some(x) = x {
                    adjacent[i] = self
                        .snapshot
                        .seats
                        .get((y as i32 + y1) as usize)
                        .and_then(|v| v.get(x).and_then(|o| o.as_ref()));
                }
                i += 1;
            }
        }
        Some((mutable.unwrap(), current.unwrap(), adjacent))
    }

    fn next_any(&mut self) -> Option<Item<'_>> {
        let current = self
            .snapshot
            .seats
            .get(self.point.1)
            .and_then(|v| v.get(self.point.0));
        let x = self.point.0;
        let y = self.point.1;
        self.point.0 += 1;
        if self.point.0 >= self.snapshot.max_x {
            self.point.0 = 0;
            self.point.1 += 1;
        }
        if current.is_none() {
            return None;
        }
        let mutable = self.grid.seats.get_mut(y).and_then(|v| v.get_mut(x));
        let mut adjacent = [None; 8];
        let mut i = 0;
        for y1 in -1i32..=1 {
            for x1 in -1i32..=1 {
                if x1 == 0 && y1 == 0 {
                    continue;
                }
                let mut tmp_pos = (x as isize, y as isize);
                loop {
                    let x = {
                        if x1 < 0 && tmp_pos.0 == 0
                            || x1 > 0 && tmp_pos.0 == (self.snapshot.max_x - 1) as isize
                            || y1 < 0 && tmp_pos.1 == 0
                            || y1 > 0 && tmp_pos.1 == (self.snapshot.seats.len() - 1) as isize
                        {
                            None
                        } else {
                            tmp_pos.0 += x1 as isize;
                            tmp_pos.1 += y1 as isize;
                            Some(tmp_pos.0)
                        }
                    };
                    match x {
                        Some(x) => {
                            match self
                                .snapshot
                                .seats
                                .get(tmp_pos.1 as usize)
                                .and_then(|v| v.get(x as usize).and_then(|o| o.as_ref()))
                            {
                                Some(seat) => {
                                    adjacent[i] = Some(seat);
                                    break;
                                }
                                None => {}
                            }
                        }
                        None => break,
                    }
                }
                i += 1;
            }
        }
        Some((mutable.unwrap(), current.unwrap(), adjacent))
    }
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let mut src = vec![];
        let mut max_x = 0;
        for line in input.lines() {
            let chars: Vec<char> = line.trim().chars().collect();
            if max_x == 0 {
                max_x = chars.len();
            }
            src.push(
                chars
                    .into_iter()
                    .map(|c| match c {
                        '.' => None,
                        'L' => Some(Seat {
                            vacancy: SeatVacancy::Empty,
                        }),
                        '#' => Some(Seat {
                            vacancy: SeatVacancy::Occupied,
                        }),
                        _ => panic!("Invalid char"),
                    })
                    .collect(),
            );
        }
        Grid { seats: src, max_x }
    }

    fn iter_mut(&mut self) -> GridIter<'_> {
        GridIter {
            snapshot: self.clone(),
            grid: self,
            point: (0, 0),
        }
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    loop {
        let mut iter = grid.iter_mut();
        let mut changed = 0;
        while let Some((mutref, shared, adjacent)) = iter.next() {
            if mutref.is_some() {
                match (
                    shared,
                    adjacent
                        .iter()
                        .flatten()
                        .filter(|s| s.vacancy == SeatVacancy::Occupied)
                        .count(),
                ) {
                    (Some(seat), 0) if seat.vacancy == SeatVacancy::Empty => {
                        mutref.as_mut().unwrap().vacancy = SeatVacancy::Occupied
                    }
                    (Some(seat), 4..=8) if seat.vacancy == SeatVacancy::Occupied => {
                        mutref.as_mut().unwrap().vacancy = SeatVacancy::Empty
                    }
                    _ => {
                        continue;
                    }
                }
                changed += 1;
            }
        }
        if changed == 0 {
            return grid
                .seats
                .iter()
                .flat_map(|s| s.iter())
                .flat_map(|s| s)
                .filter(|s| s.vacancy == SeatVacancy::Occupied)
                .count();
        }
    }
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    loop {
        let mut iter = grid.iter_mut();
        let mut changed = 0;
        while let Some((mutref, shared, adjacent)) = iter.next_any() {
            if mutref.is_some() {
                match (
                    shared,
                    adjacent
                        .iter()
                        .flatten()
                        .filter(|s| s.vacancy == SeatVacancy::Occupied)
                        .count(),
                ) {
                    (Some(seat), 0) if seat.vacancy == SeatVacancy::Empty => {
                        mutref.as_mut().unwrap().vacancy = SeatVacancy::Occupied
                    }
                    (Some(seat), 5..=8) if seat.vacancy == SeatVacancy::Occupied => {
                        mutref.as_mut().unwrap().vacancy = SeatVacancy::Empty
                    }
                    _ => {
                        continue;
                    }
                }
                changed += 1;
            }
        }
        if changed == 0 {
            return grid
                .seats
                .iter()
                .flat_map(|s| s.iter())
                .flat_map(|s| s)
                .filter(|s| s.vacancy == SeatVacancy::Occupied)
                .count();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        // L.L#
        // #..L
        let grid = "L.L#
#..L";
        let mut grid = Grid::parse(grid);
        let mut iter = grid.iter_mut();
        let entry = iter.next();
        assert_eq!(
            1,
            entry
                .unwrap()
                .2
                .iter()
                .flatten()
                .filter(|s| s.vacancy == SeatVacancy::Occupied)
                .count()
        );
        let mut iter = grid.iter_mut();
        let mut cnt = 0;
        while let Some(_) = iter.next() {
            cnt += 1;
        }
        assert_eq!(cnt, 8);
    }

    #[test]
    fn part1_example() {
        let grid = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(part1(grid), 37);
    }

    #[test]
    fn part2_example() {
        let grid = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(part2(grid), 26);
    }
}

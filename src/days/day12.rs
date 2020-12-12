#[derive(Debug)]
struct Location {
    x: i32, // east/west
    y: i32, // north/south
    facing: Facing,
    ship: Option<Box<Location>>,
}

#[derive(Copy, Clone, Debug)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    #[inline]
    fn turn(self, direction: char, degrees: i32) -> Facing {
        match (self, degrees, direction) {
            (x, 270, 'L') => x.turn('R', 90),
            (x, 270, 'R') => x.turn('L', 90),
            (Facing::North, 180, _) => Facing::South,
            (Facing::South, 180, _) => Facing::North,
            (Facing::East, 180, _) => Facing::West,
            (Facing::West, 180, _) => Facing::East,
            (Facing::North, 90, 'L') => Facing::West,
            (Facing::North, 90, 'R') => Facing::East,
            (Facing::South, 90, 'L') => Facing::East,
            (Facing::South, 90, 'R') => Facing::West,
            (Facing::East, 90, 'L') => Facing::North,
            (Facing::East, 90, 'R') => Facing::South,
            (Facing::West, 90, 'L') => Facing::South,
            (Facing::West, 90, 'R') => Facing::North,
            (a, b, c) => panic!("Invalid direction/value {:?} {} {}", a, b, c),
        }
    }

    fn step(self) -> char {
        match self {
            Facing::North => 'N',
            Facing::South => 'S',
            Facing::East => 'E',
            Facing::West => 'W',
        }
    }
}

impl Location {
    fn new() -> Location {
        Location {
            x: 0,
            y: 0,
            facing: Facing::East,
            ship: None,
        }
    }

    fn with_ship() -> Location {
        Location {
            x: 10,
            y: 1,
            facing: Facing::East,
            ship: Some(Box::new(Location::new())),
        }
    }

    fn step(&mut self, action: char, value: i32) {
        match action {
            'N' => self.y += value,
            'S' => self.y -= value,
            'E' => self.x += value,
            'W' => self.x -= value,
            'F' => match &mut self.ship {
                Some(ship) => {
                    ship.x += self.x * value;
                    ship.y += self.y * value;
                }
                None => self.step(self.facing.step(), value),
            },
            c => match &self.ship {
                Some(_) => match (c, value) {
                    (_, 180) => {
                        self.x = -self.x;
                        self.y = -self.y;
                    }
                    ('R', 270) | ('L', 90) => {
                        std::mem::swap(&mut self.x, &mut self.y);
                        self.x = -self.x;
                    }
                    ('L', 270) | ('R', 90) => {
                        std::mem::swap(&mut self.x, &mut self.y);
                        self.y = -self.y;
                    }
                    _ => panic!("Invalid direction/degrees"),
                },
                None => self.facing = self.facing.turn(c, value),
            },
        }
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> i32 {
    let mut loc = Location::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let action = chars.next().unwrap();
        let value = chars.collect::<String>().parse().unwrap();
        loc.step(action, value);
    }
    loc.x.abs() + loc.y.abs()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i32 {
    let mut loc = Location::with_ship();
    for line in input.lines() {
        let mut chars = line.chars();
        let action = chars.next().unwrap();
        let value = chars.collect::<String>().parse().unwrap();
        loc.step(action, value);
    }
    let ship = loc.ship.unwrap();
    ship.x.abs() + ship.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(part1(input), 25);
    }

    #[test]
    fn part2_example() {
        let input = "F10
N3
F7
R90
F11";
        assert_eq!(part2(input), 286);
    }
}

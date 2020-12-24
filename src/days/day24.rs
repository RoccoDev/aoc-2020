use fxhash::{FxHashMap, FxHashSet};
use strum::IntoEnumIterator;

type Tile = (i32, i32);

struct Grid {
    to_enable: Vec<Vec<Direction>>,
    states: FxHashMap<Tile, bool>,
    blacks: FxHashSet<Tile>,
}

#[derive(Debug, strum::EnumIter)]
enum Direction {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    East,
    West,
}

impl Direction {
    fn all(input: &str) -> Vec<Direction> {
        let mut chars = input.chars();
        let mut res = Vec::with_capacity(input.len());
        while let Some(ch) = chars.next() {
            match ch {
                'e' => res.push(Direction::East),
                'w' => res.push(Direction::West),
                's' => match chars.next().unwrap() {
                    'e' => res.push(Direction::SouthEast),
                    'w' => res.push(Direction::SouthWest),
                    c => panic!("Invalid direction {}", c),
                },
                'n' => match chars.next().unwrap() {
                    'e' => res.push(Direction::NorthEast),
                    'w' => res.push(Direction::NorthWest),
                    c => panic!("Invalid direction {}", c),
                },
                c => panic!("Invalid direction {}", c),
            }
        }
        res
    }

    fn step(&self) -> (i32, i32) {
        match self {
            // https://www.redblobgames.com/grids/hexagons/#coordinates-axial
            Direction::NorthEast => (1, -1),
            Direction::NorthWest => (0, -1),
            Direction::SouthEast => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

impl Grid {
    fn enable_all(&mut self) {
        for directions in self.to_enable.drain(..) {
            let mut tile = (0, 0);
            directions.iter().for_each(|d| {
                let step = d.step();
                tile.0 += step.0;
                tile.1 += step.1;
            });
            *self.states.entry(tile).or_insert(true) ^= true;
        }
    }

    fn adjacent_tiles(&self, tile: &Tile, snapshot: &FxHashSet<Tile>) -> Vec<(Tile, bool)> {
        Direction::iter()
            .map(|dir| {
                let step = dir.step();
                let tile = (tile.0 + step.0, tile.1 + step.1);
                (tile, !snapshot.contains(&tile))
            })
            .collect()
    }

    fn flip_all(&mut self) {
        self.blacks = self
            .states
            .iter()
            .filter_map(|(k, &v)| if !v { Some(*k) } else { None })
            .collect();
        for _ in 0..100 {
            let snapshot = self.blacks.clone();
            let mut possible_whites = vec![];
            for old in &snapshot {
                let (white_neighbours, black_neighbours): (Vec<(Tile, bool)>, Vec<(Tile, bool)>) =
                    self.adjacent_tiles(old, &snapshot)
                        .iter()
                        .partition(|(_, v)| *v);
                if black_neighbours.len() == 0 || black_neighbours.len() > 2 {
                    self.blacks.remove(old);
                }
                possible_whites.extend(white_neighbours.into_iter().map(|(tile, _)| tile));
            }
            // See Day 17
            possible_whites.sort();
            let mut index = 0;
            while index < possible_whites.len() as isize - 2 {
                let idx = index as usize;
                let src = possible_whites[idx].clone();
                let mut occurrences = 0;
                while possible_whites[idx + occurrences] == src {
                    occurrences += 1;
                }
                if occurrences == 2 {
                    self.blacks.insert(src);
                }
                index += occurrences as isize;
            }
        }
    }
}

fn parse_input(input: &str) -> Grid {
    Grid {
        states: FxHashMap::default(),
        to_enable: input.lines().map(|l| Direction::all(l)).collect(),
        blacks: FxHashSet::default(),
    }
}

#[aoc(day24, part1)]
fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    grid.enable_all();
    grid.states.values().filter(|&&v| !v).count()
}

#[aoc(day24, part2)]
fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    grid.enable_all();
    grid.flip_all();
    grid.blacks.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(part1(input), 10);
    }

    #[test]
    fn part2_example() {
        let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(part2(input), 2208);
    }
}

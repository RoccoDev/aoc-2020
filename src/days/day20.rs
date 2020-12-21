use fxhash::FxHashMap;
use regex::Regex;

lazy_static::lazy_static! {
    static ref TILE: Regex = Regex::new(r#"Tile (\d+):"#).unwrap();
}

#[derive(Debug)]
struct Tile {
    id: u64,
    borders: [u64; 4],
}

// INCOMPLETE: Skipping part 2 for now
/// This time the tile can be represented as an 8x8 bitboard
struct StrippedTile(u64);

impl StrippedTile {
    fn flip_vert(&mut self) {
        self.0 = self.0 ^ 56;
    }

    fn flip_hori(&mut self) {
        self.0 = self.0 ^ 7;
    }

    fn rotate_clockwise(&mut self) {
        self.0 = (((self.0 >> 3) | (self.0 << 3)) & 63) ^ 56;
        self.0 = ((self.0 * 0x20800000) >> 26) ^ 56;
    }

    fn rotate_counterclockwise(&mut self) {
        self.0 = (((self.0 >> 3) | (self.0 << 3)) & 63) ^ 7;
        self.0 = ((self.0 * 0x20800000) >> 26) ^ 7;
    }
}

impl Tile {
    fn flip(mut i: u64) -> u64 {
        let mut res = 0;
        for _ in 0..10 {
            res <<= 1;
            if (i & 1) == 1 {
                res |= 1;
            }
            i >>= 1;
        }
        res
    }

    fn parse_blob(id: u64, lines: Vec<&str>) -> Tile {
        let mut borders = [0; 4];
        let to_parse: [Vec<char>; 4] = [
            lines[0].chars().collect(),
            lines.last().unwrap().chars().collect(),
            lines.iter().map(|l| l.chars().next().unwrap()).collect(),
            lines.iter().map(|l| l.chars().last().unwrap()).collect(),
        ];
        for (i, border) in to_parse.iter().enumerate() {
            let mut bord = 0;
            for c in border {
                bord <<= 1;
                bord |= {
                    if *c == '#' {
                        1
                    } else {
                        0
                    }
                };
            }
            borders[i] = bord.min(Tile::flip(bord));
        }
        Tile { id, borders }
    }
}

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|p| {
            let mut lines = p.lines();
            let title = lines.next().unwrap();
            let id = TILE.captures(title).unwrap()[1].parse().unwrap();
            let other = lines.collect();
            Tile::parse_blob(id, other)
        })
        .collect()
}

#[aoc(day20, part1)]
fn part1(input: &str) -> u64 {
    let tiles = parse_input(input);
    let mut edges = FxHashMap::default();
    for tile in &tiles {
        for edge in &tile.borders {
            edges.entry(*edge).or_insert(vec![]).push(tile.id);
        }
    }
    tiles
        .iter()
        .filter(|tile| {
            tile.borders
                .iter()
                .filter(|edge| edges[edge].len() == 2)
                .count()
                == 2
        })
        .map(|tile| tile.id)
        .product()
}

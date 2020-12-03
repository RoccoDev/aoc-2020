struct Map {
    src: Vec<Vec<char>>,
    max_x: usize,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut src = vec![];
        let mut max_x = 0;
        for line in input.lines() {
            let lines: Vec<char> = line.trim().chars().collect();
            if max_x == 0 {
                max_x = lines.len();
            }
            src.push(lines);
        }
        Map { src, max_x }
    }

    /// Returns the element at the given position.
    /// It will automatically overlap (on the right side) and return `None` when the bottom of the map is reached.
    fn get(&self, x: usize, y: usize) -> Option<&char> {
        if y >= self.src.len() {
            return None;
        }
        self.src
            .get(y % self.src.len())
            .and_then(|v| v.get(x % self.max_x))
    }
}

fn calc_trees(map: &Map, slope: (usize, usize)) -> i32 {
    let mut last = (slope.0, slope.1);
    let mut count = 0;
    while let Some(ch) = map.get(last.0, last.1) {
        if *ch == '#' {
            count += 1;
        }
        last.0 += slope.0;
        last.1 += slope.1;
    }
    count
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let map = Map::parse(input);
    calc_trees(&map, (3, 1))
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let map = Map::parse(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1u32, |v, slope| v * calc_trees(&map, *slope) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let example = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";
        assert_eq!(part1(example), 7);
    }

    #[test]
    fn part2_example() {
        let example = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";
        assert_eq!(part2(example), 336);
    }
}

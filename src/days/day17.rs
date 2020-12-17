use fxhash::FxHashSet;

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
struct Cube(i8, i8, i8, i8);

fn initial_active_cubes(input: &str) -> FxHashSet<Cube> {
    let mut cubes = FxHashSet::default();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|c| c.1 == '#')
            .for_each(|(x, _)| {
                cubes.insert(Cube(x as i8, y as i8, 0, 0));
            });
    });
    cubes
}

fn calc(input: &str, use_w: bool) -> usize {
    // We only care about active cubes
    let mut actives = initial_active_cubes(input);
    for _ in 0..6 {
        // Save the grid's state
        let snapshot = actives.clone();
        let mut known_neighbours = vec![];
        // Here, for each cube in the previous state, find the neighbours.
        // If the neighbours are active, increment the counter and check if it's not 2 or 3 to deactivate the current cube.
        // If the neighbours are inactive, add them to a list of known inactive neighbours.
        for old in &snapshot {
            let mut neighbour_count = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    for z in -1..=1 {
                        for w in {
                            if use_w {
                                -1..=1
                            } else {
                                0..=0
                            }
                        } {
                            if [x, y, z, w].iter().all(|&x| x == 0) {
                                continue;
                            }
                            let neighbour = Cube(old.0 + x, old.1 + y, old.2 + z, old.3 + w);
                            if snapshot.contains(&neighbour) {
                                neighbour_count += 1;
                            } else {
                                known_neighbours.push(neighbour);
                            }
                        }
                    }
                }
            }
            if neighbour_count != 2 && neighbour_count != 3 {
                actives.remove(old);
            }
        }
        // Sort the neighbours so duplicates will appear one next to each other.
        // For example, suppose the inactive cube (x, y, z) has 3 active neighbours.
        // The element will appear as (x, y, z); (x, y, z); (x, y, z). This makes it easy to count the occurrences
        known_neighbours.sort();
        let mut index = 0;
        while index < known_neighbours.len() as isize - 3 {
            let idx = index as usize;
            let src = known_neighbours[idx].clone();
            let mut occurrences = 0;
            while known_neighbours[idx + occurrences] == src {
                occurrences += 1;
            }
            if occurrences == 3 {
                actives.insert(src);
            }
            index += occurrences as isize;
        }
    }
    actives.len()
}

#[aoc(day17, part1)]
fn part1(input: &str) -> usize {
    calc(input, false)
}

#[aoc(day17, part2)]
fn part2(input: &str) -> usize {
    calc(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = ".#.
..#
###";
        assert_eq!(part1(input), 112);
    }

    #[test]
    fn part2_example() {
        let input = ".#.
..#
###";
        assert_eq!(part2(input), 848);
    }
}

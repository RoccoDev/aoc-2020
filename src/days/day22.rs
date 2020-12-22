use std::collections::VecDeque;

use fxhash::FxHashSet;

struct Game {
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
    needed: usize,
}

struct GameState {
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
    known1: FxHashSet<VecDeque<u8>>,
    known2: FxHashSet<VecDeque<u8>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum RoundResult {
    Player1,
    Player2,
    Continue,
}

impl Game {
    fn step_round(&mut self) -> RoundResult {
        let (choice1, choice2) = (
            self.player1.pop_front().unwrap(),
            self.player2.pop_front().unwrap(),
        );
        let winner = {
            if choice1 > choice2 {
                self.player1.push_back(choice1);
                self.player1.push_back(choice2);
                (RoundResult::Player1, &self.player1)
            } else {
                self.player2.push_back(choice2);
                self.player2.push_back(choice1);
                (RoundResult::Player2, &self.player2)
            }
        };
        if winner.1.len() == self.needed {
            winner.0
        } else {
            RoundResult::Continue
        }
    }

    fn recursive_match(state: &GameState, choice1: u8, choice2: u8) -> RoundResult {
        let player1: VecDeque<_> = state
            .player1
            .iter()
            .copied()
            .take(choice1 as usize)
            .collect();
        let player2: VecDeque<_> = state
            .player2
            .iter()
            .copied()
            .take(choice2 as usize)
            .collect();
        let needed = player1.len() + player2.len();
        let mut state = GameState {
            player1,
            player2,
            known1: FxHashSet::default(),
            known2: FxHashSet::default(),
        };
        loop {
            match Game::step_recursive(&mut state, needed) {
                RoundResult::Continue => {}
                r => return r,
            }
        }
    }

    fn step_recursive(state: &mut GameState, needed: usize) -> RoundResult {
        if state.known1.contains(&state.player1) && state.known2.contains(&state.player2) {
            return RoundResult::Player1;
        }
        state.known1.insert(state.player1.clone());
        state.known2.insert(state.player2.clone());
        let (choice1, choice2) = (
            state.player1.pop_front().unwrap(),
            state.player2.pop_front().unwrap(),
        );
        let winner = {
            if state.player1.len() >= choice1 as usize && state.player2.len() >= choice2 as usize {
                Game::recursive_match(state, choice1, choice2)
            } else if choice1 > choice2 {
                RoundResult::Player1
            } else {
                RoundResult::Player2
            }
        };
        match winner {
            RoundResult::Player1 => {
                state.player1.push_back(choice1);
                state.player1.push_back(choice2);
            }
            RoundResult::Player2 => {
                state.player2.push_back(choice2);
                state.player2.push_back(choice1);
            }
            RoundResult::Continue => {}
        }
        if {
            if winner == RoundResult::Player1 {
                &state.player1
            } else {
                &state.player2
            }
        }
        .len()
            == needed
        {
            winner
        } else {
            RoundResult::Continue
        }
    }

    fn play(&mut self) -> usize {
        loop {
            let winner = match self.step_round() {
                RoundResult::Player1 => &self.player1,
                RoundResult::Player2 => &self.player2,
                RoundResult::Continue => continue,
            };
            return winner
                .iter()
                .rev()
                .enumerate()
                .map(|(i, v)| (i + 1) * *v as usize)
                .sum();
        }
    }

    fn play_recursive(self) -> usize {
        let needed = self.needed;
        let mut state = GameState {
            known1: FxHashSet::default(),
            known2: FxHashSet::default(),
            player1: self.player1,
            player2: self.player2,
        };
        loop {
            let winner = match Game::step_recursive(&mut state, needed) {
                RoundResult::Player1 => &state.player1,
                RoundResult::Player2 => &state.player2,
                RoundResult::Continue => continue,
            };
            return winner
                .iter()
                .rev()
                .enumerate()
                .map(|(i, v)| (i + 1) * *v as usize)
                .sum();
        }
    }
}

fn parse_input(input: &str) -> Game {
    let mut parts = input.split("\n\n");
    let player1: VecDeque<_> = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();
    let player2: VecDeque<_> = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();
    Game {
        needed: player1.len() + player2.len(),
        player1,
        player2,
    }
}

#[aoc(day22, part1)]
fn part1(input: &str) -> usize {
    parse_input(input).play()
}

#[aoc(day22, part2)]
fn part2(input: &str) -> usize {
    parse_input(input).play_recursive()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(part1(input), 306);
    }

    #[test]
    fn part2_example() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(part2(input), 291);
    }
}

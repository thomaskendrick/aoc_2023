use std::cmp::Ordering;
use std::u32;

use itertools::Itertools;

#[derive(Debug, Eq)]
struct Game {
    hand: Vec<u32>,
    bid: u32,
    wildcards: bool,
}

impl Game {
    fn from_line(input: &str, wildcards: bool) -> Self {
        let (card_str, bid_str) = input.split_once(' ').unwrap();
        let hand: Vec<_> = card_str
            .chars()
            .map(|c| match c {
                c if c.is_ascii_digit() => c.to_digit(10).unwrap(),
                'T' => 10,
                'J' => match wildcards {
                    true => 1,
                    false => 11,
                },
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unimplemented!(),
            })
            .collect();

        Self {
            hand,
            bid: bid_str.parse().unwrap(),
            wildcards,
        }
    }
    fn hand_strength(&self) -> u32 {
        let mut counts = self.hand.iter().counts();
        if self.wildcards {
            let jokers = *counts.get(&1).unwrap_or(&0);
            if jokers > 0 && jokers < 5 {
                let mut highest = (0u32, 0usize);
                for (k, v) in counts.iter() {
                    if **k != 1 && (v > &highest.1 || (k > &&highest.0 && v >= &highest.1)) {
                        highest = (**k, *v);
                    }
                }
                let highest_value = counts.get_mut(&highest.0).unwrap();
                if highest.0 != 1 {
                    *highest_value += jokers;
                    counts.remove(&1u32);
                }
            }
        }
        match counts.len() {
            1 => 7,
            2 => {
                if counts.iter().any(|(_, v)| *v == 4) {
                    6
                } else {
                    5
                }
            }
            3 => {
                if counts.iter().any(|(_, v)| *v == 3) {
                    4
                } else {
                    3
                }
            }
            4 => 2,
            5 => 1,
            _ => panic!("Too many cards!"),
        }
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        let hs = self.hand_strength();
        let ohs = other.hand_strength();

        if hs != ohs {
            return hs.cmp(&ohs);
        }

        for i in 0..5 {
            if self.hand[i] != other.hand[i] {
                return self.hand[i].cmp(&other.hand[i]);
            }
        }
        Ordering::Equal
    }
}

fn part1(input: &str) -> u32 {
    let mut games: Vec<_> = input
        .lines()
        .map(|str| Game::from_line(str, false))
        .collect();
    games.sort();
    games
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, game)| acc + (game.bid * (i as u32 + 1)))
}

fn part2(input: &str) -> u32 {
    let mut games: Vec<_> = input
        .lines()
        .map(|str| Game::from_line(str, true))
        .collect();
    games.sort();
    games
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, game)| acc + (game.bid * (i as u32 + 1)))
}

fn main() {
    let input = include_str!("../input/day7.txt");
    aoc2023::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day7.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 6440);
    }
    #[test]
    fn cmp_test() {
        let game1 = Game {
            hand: vec![13, 10, 1, 1, 10],
            bid: 0,
            wildcards: true,
        };
        let game2 = Game {
            hand: vec![12, 12, 12, 1, 14],
            bid: 0,
            wildcards: true,
        };
        assert_eq!(game1.cmp(&game2), Ordering::Greater);
    }
    #[test]
    fn hand_strength_test_wildcards() {
        let game = Game {
            hand: vec![12, 2, 13, 1, 1],
            bid: 0,
            wildcards: true,
        };
        assert_eq!(game.hand_strength(), 4);
    }
    #[test]
    fn hand_strength_test_fiveofakind() {
        let game = Game {
            hand: vec![2, 2, 2, 2, 2],
            bid: 0,
            wildcards: false,
        };
        assert_eq!(game.hand_strength(), 7);
    }
    #[test]
    fn hand_strength_test_fourofakind() {
        let game = Game {
            hand: vec![2, 2, 3, 2, 2],
            bid: 0,
            wildcards: false,
        };
        assert_eq!(game.hand_strength(), 6);
    }
    #[test]
    fn hand_strength_test_fullhouse() {
        let game = Game {
            hand: vec![4, 2, 4, 2, 2],
            bid: 0,
            wildcards: false,
        };
        assert_eq!(game.hand_strength(), 5);
    }
    #[test]
    fn hand_strength_test_threeofakind() {
        let game = Game {
            hand: vec![4, 2, 4, 7, 4],
            bid: 0,
            wildcards: false,
        };
        assert_eq!(game.hand_strength(), 4);
    }
    #[test]
    fn hand_strength_test_twopair() {
        let game = Game {
            hand: vec![2, 2, 4, 7, 4],
            bid: 0,
            wildcards: false,
        };
        assert_eq!(game.hand_strength(), 3);
    }
    #[test]
    fn hand_strength_test_pair() {
        let game = Game {
            hand: vec![2, 6, 4, 7, 4],
            bid: 0,
            wildcards: false,
        };
        assert_eq!(game.hand_strength(), 2);
    }
    #[test]
    fn hand_strength_test_highcard() {
        let game = Game {
            hand: vec![2, 6, 7, 1, 4],
            bid: 0,
            wildcards: false,
        };
        assert_eq!(game.hand_strength(), 1);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 5905);
    }
}

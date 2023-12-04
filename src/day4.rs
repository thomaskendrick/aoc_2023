#[derive(Debug)]
struct Card {
    left_numbers: Vec<i32>,
    right_numbers: Vec<i32>,
}

impl Card {
    fn from_line(input: &str) -> Self {
        let (_, numbers_str) = input.split_once(':').unwrap();
        let (lhs_str, rhs_str) = numbers_str.split_once('|').unwrap();
        let left_numbers: Vec<i32> = lhs_str
            .split_whitespace()
            .map(|str| str.parse().unwrap())
            .collect();
        let right_numbers: Vec<i32> = rhs_str
            .split_whitespace()
            .map(|str| str.trim().parse().unwrap())
            .collect();
        Card {
            left_numbers,
            right_numbers,
        }
    }
    fn matches(&self) -> usize {
        self.left_numbers
            .iter()
            .filter(|n| self.right_numbers.contains(n))
            .count()
    }
    fn points(&self) -> u32 {
        let matches = self.matches();
        if self.matches() == 0 {
            return 0;
        }
        2u32.pow(matches as u32 - 1)
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Card::from_line)
        .map(|card| card.points())
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut cards: Vec<(Card, u32)> = input.lines().map(|ln| (Card::from_line(ln), 1)).collect();
    for i in 0..cards.len() {
        let matches = cards[i].0.matches();
        let current_copies = cards[i].1;
        let id = i + 1;
        for below_card_id in id + 1..id + 1 + matches {
            let (_, copies) = cards.get_mut(below_card_id - 1).unwrap();
            *copies += current_copies;
        }
    }
    cards.iter().map(|(_, copies)| copies).sum()
}

fn main() {
    let input = include_str!("../input/day4.txt");
    aoc2023::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day4.txt");
    #[test]
    fn pow_test() {
        let base = 2u32;
        assert_eq!(base.pow(1), 2);
    }
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 30);
    }
}

fn part1(input: &str) -> i32 {
    0
}

fn part2(input: &str) -> i32 {
    0
}

fn main() {
    let input = include_str!("../input/day9.txt");
    aoc2023::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day9.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}

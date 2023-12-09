enum Direction {
    Forward,
    Backward,
}

fn parse_line(input: &str) -> Vec<isize> {
    input
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect()
}

fn generate_differences(values: Vec<isize>) -> Vec<Vec<isize>> {
    let mut differences = vec![values];
    while !differences.last().unwrap().iter().all(|v| *v == 0isize) {
        differences.push(
            differences
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect(),
        );
    }
    differences
}

fn extrapolate_from_differences(differences: Vec<Vec<isize>>, direction: Direction) -> isize {
    differences
        .into_iter()
        .rev()
        .skip(1)
        .fold(0isize, |acc, v| match direction {
            Direction::Forward => v.last().unwrap() + acc,
            Direction::Backward => v.first().unwrap() - acc,
        })
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(parse_line)
        .map(generate_differences)
        .map(|d| extrapolate_from_differences(d, Direction::Forward))
        .sum()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(parse_line)
        .map(generate_differences)
        .map(|d| extrapolate_from_differences(d, Direction::Backward))
        .sum()
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
        assert_eq!(part1(EXAMPLE), 114);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 2);
    }
    #[test]
    fn generate_differences_test() {
        assert_eq!(
            generate_differences(vec![1, 3, 6, 10, 15, 21],),
            vec![
                vec![1, 3, 6, 10, 15, 21],
                vec![2, 3, 4, 5, 6],
                vec![1, 1, 1, 1],
                vec![0, 0, 0]
            ]
        );
    }
    #[test]
    fn extrapolate_from_differences_test() {
        assert_eq!(
            extrapolate_from_differences(
                generate_differences(vec![1, 3, 6, 10, 15, 21]),
                Direction::Forward
            ),
            28
        );
    }
}

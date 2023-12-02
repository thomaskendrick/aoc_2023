const NUMBER_STRINGS: [(&str, u32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];
fn solver(input: &str, match_words: bool) -> u32 {
    input
        .lines()
        .map(|ln| {
            let mut f = None;
            let mut l = None;

            for (i, c) in ln.chars().enumerate() {
                if c.is_ascii_digit() {
                    if f.is_none() {
                        f = Some(c.to_digit(10).unwrap());
                    }
                    l = Some(c.to_digit(10).unwrap())
                } else if match_words {
                    for (k, v) in NUMBER_STRINGS {
                        if ln[i..].starts_with(k) {
                            if f.is_none() {
                                f = Some(v);
                            }
                            l = Some(v)
                        }
                    }
                }
            }
            (f.unwrap() * 10) + l.unwrap()
        })
        .sum()
}

fn part1(input: &str) -> u32 {
    solver(input, false)
}
fn part2(input: &str) -> u32 {
    solver(input, true)
}

fn main() {
    let input = include_str!("../input/day1.txt");
    aoc2023::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_PT1: &str = include_str!("../example/day1_pt1.txt");
    const EXAMPLE_PT2: &str = include_str!("../example/day1_pt2.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE_PT1), 142);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE_PT2), 281);
    }
}

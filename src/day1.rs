fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let mut first_digit = None;
            let mut last_digit = None;

            for char in l.chars() {
                if char.is_digit(10) && first_digit.is_none() {
                    first_digit = Some(char);
                }
                if char.is_digit(10) {
                    last_digit = Some(char)
                }
            }
            let s1 = first_digit.unwrap().to_string();
            let s2 = last_digit.unwrap().to_string();
            (s1 + &s2).parse::<i32>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    return 0;
}

fn main() {
    let input = include_str!("../input/day1.txt");
    aoc2022::solve_puzzles(input, part1, part2)
}

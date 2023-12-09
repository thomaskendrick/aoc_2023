use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

type Branch<'a> = (&'a str, &'a str);

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, Branch>) {
    let (direction_str, branches_str) = input.split_once("\n\n").unwrap();
    let directions: Vec<Direction> = direction_str
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        })
        .collect();

    let map: HashMap<&str, Branch> = branches_str
        .lines()
        .map(|ln| {
            let (source_str, dests_str) = ln.split_once(" = (").unwrap();
            let (left_target, right_target) = dests_str
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();
            (source_str, (left_target, right_target))
        })
        .collect();
    (directions, map)
}

fn part1(input: &str) -> u32 {
    let (directions, map) = parse_input(input);

    let mut steps: u32 = 0;
    let mut next_step = "AAA";

    for direction in directions.iter().cycle() {
        steps += 1;

        let (left_choice, right_choice) = map.get(next_step).unwrap();
        match direction {
            Direction::Left => next_step = left_choice,
            Direction::Right => next_step = right_choice,
        }
        if next_step == "ZZZ" {
            break;
        }
    }
    steps
}

fn part2(input: &str) -> usize {
    let (directions, map) = parse_input(input);

    let mut steps: usize = 0;
    let mut points: Vec<(&str, Option<u32>)> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .map(|k| (k, None))
        .collect();

    for direction in directions.iter().cycle() {
        steps += 1;
        for (point, cycle) in points.iter_mut() {
            let (left_choice, right_choice) = map.get(point).unwrap();
            match direction {
                Direction::Left => *point = left_choice,
                Direction::Right => *point = right_choice,
            }
            if point.ends_with('Z') {
                *cycle = Some(steps as u32);
            }
        }
        if points.iter().all(|(_, cycle)| cycle.is_some()) {
            break;
        }
    }
    points
        .iter()
        .map(|(_, steps)| steps.unwrap())
        .fold(points.first().unwrap().1.unwrap() as usize, |acc, val| {
            lcm(acc, val as usize)
        })
}

fn main() {
    let input = include_str!("../input/day8.txt");
    aoc2023::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day8_1.txt");
    const EXAMPLE_2: &str = include_str!("../example/day8_2.txt");
    const EXAMPLE_3: &str = include_str!("../example/day8_3.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 2);
    }
    #[test]
    fn part_1_test_2() {
        assert_eq!(part1(EXAMPLE_2), 6);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE_3), 6);
    }
}

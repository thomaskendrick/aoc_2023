type Races = Vec<Race>;

#[derive(Debug)]
struct Race {
    record: usize,
    time: usize,
}

impl Race {
    fn get_distances(&self) -> Vec<usize> {
        (0..=self.time).map(|n| n * (self.time - n)).collect()
    }
    fn get_distance_count_with_boundries(&self) -> usize {
        let mut start_winning: Option<usize> = None;
        let mut finish_winning: Option<usize> = None;
        for n in 0..=self.time {
            let distance = n * (self.time - n);
            if distance > self.record {
                start_winning = Some(n);
                break;
            }
        }
        for n in (0..=self.time).rev() {
            let distance = n * (self.time - n);
            if distance > self.record {
                finish_winning = Some(n);
                break;
            }
        }
        finish_winning.unwrap() - start_winning.unwrap() + 1
    }
}

fn parse_line(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Races {
    let mut lines = input.lines();
    let times = parse_line(lines.next().unwrap());
    let distance = parse_line(lines.next().unwrap());
    times
        .into_iter()
        .zip(distance)
        .map(|(time, record)| Race { record, time })
        .collect()
}
fn parse_big_input(input: &str) -> Race {
    let mut lines = input.lines();
    let time: usize = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(char::is_whitespace, "")
        .parse()
        .unwrap();
    let distance: usize = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .replace(char::is_whitespace, "")
        .parse()
        .unwrap();

    Race {
        record: distance,
        time,
    }
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|r| {
            r.get_distances()
                .into_iter()
                .filter(|d| *d > r.record)
                .count()
        })
        .product()
}

fn part2(input: &str) -> usize {
    let race = parse_big_input(input);
    race.get_distance_count_with_boundries()
}

fn main() {
    let input = include_str!("../input/day6.txt");
    aoc2023::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day6.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 288);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 71503);
    }
}

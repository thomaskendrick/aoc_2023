use rayon::prelude::*;
use std::{collections::HashMap, ops::Range, ops::RangeInclusive};

#[derive(Debug)]
struct GardenMap<'a> {
    destination: &'a str,
    ranges: Vec<(usize, RangeInclusive<usize>)>,
}

impl<'a> GardenMap<'a> {
    fn get_location(&self, number: usize) -> (&str, usize) {
        for (location, range) in &self.ranges {
            if range.contains(&number) {
                return (&self.destination, location + (number - range.start()));
            }
        }
        (&self.destination, number)
    }
}

fn parse_input(input: &str) -> (Vec<usize>, HashMap<&str, GardenMap>) {
    let (seed_str, maps_str) = input.split_once("\n\n").unwrap();

    let seeds: Vec<usize> = seed_str
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|str| str.parse().unwrap())
        .collect();

    let maps = maps_str
        .split("\n\n")
        .map(|map_str| {
            let mut lines = map_str.lines();
            let (source, destination) = lines
                .next()
                .unwrap()
                .split_once(' ')
                .unwrap()
                .0
                .split_once("-to-")
                .unwrap();

            let ranges: Vec<(usize, RangeInclusive<usize>)> = lines
                .map(|ln| {
                    let mut num_str_splt = ln.split_whitespace();
                    let source: usize = num_str_splt.next().unwrap().parse().unwrap();
                    let start: usize = num_str_splt.next().unwrap().parse().unwrap();
                    let length: usize = num_str_splt.next().unwrap().parse().unwrap();
                    (source, start..=start + length)
                })
                .collect();
            (
                source,
                GardenMap {
                    destination,
                    ranges,
                },
            )
        })
        .collect();

    (seeds, maps)
}

fn search_for_location(source: &str, number: usize, map_list: &HashMap<&str, GardenMap>) -> usize {
    let (new_source, new_number) = map_list.get(source).unwrap().get_location(number);
    if new_source == "location" {
        return new_number;
    }
    search_for_location(new_source, new_number, map_list)
}

fn part1(input: &str) -> usize {
    let (seeds, map_list) = parse_input(input);
    seeds
        .iter()
        .map(|number| search_for_location("seed", *number, &map_list))
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let (seeds, map_list) = parse_input(input);

    let mut ranges = seeds
        .chunks(2)
        .map(|seed_pair| (seed_pair[0]..seed_pair[0] + seed_pair[1]))
        .collect::<Vec<_>>();

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut merged_ranges: Vec<Range<usize>> = Vec::new();
    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if last.end >= range.start {
                last.end = last.end.max(range.end);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
        .into_iter()
        .map(|range| {
            range
                .into_par_iter()
                .map(|number| search_for_location("seed", number, &map_list))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../input/day5.txt");
    aoc2023::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day5.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 35);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 46);
    }
}

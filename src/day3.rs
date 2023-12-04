use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct PartNumber {
    value: i32,
    start: Point,
    end: Point,
}

fn get_cell((x_pos, y_pos): Point, grid: &'_ Vec<Vec<Cell>>) -> Option<&'_ Cell> {
    if x_pos < 0 || x_pos > grid[0].len() as i32 - 1 || y_pos < 0 || y_pos > grid.len() as i32 - 1 {
        return None;
    }
    Some(&grid[y_pos as usize][x_pos as usize])
}

impl PartNumber {
    fn is_valid(&self, grid: &Vec<Vec<Cell>>) -> bool {
        let (start_x, start_y) = self.start;
        let (end_x, end_y) = self.end;

        let mut neighbors: Vec<Option<&Cell>> = vec![
            get_cell((start_x - 1, start_y), grid),
            get_cell((end_x + 1, end_y), grid),
        ];

        for x in start_x - 1..=end_x + 1 {
            neighbors.push(get_cell((x, start_y - 1), grid));
            neighbors.push(get_cell((x, start_y + 1), grid));
        }
        let result = neighbors.iter().any(|cell| {
            if let Some(Cell::Symbol(_)) = cell {
                return true;
            }
            false
        });
        result
    }
    fn includes_point(&self, point: &Point) -> bool {
        let (start_x, start_y) = self.start;
        let (end_x, _) = self.end;
        for x in start_x..=end_x {
            if (x, start_y) == *point {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
enum Cell {
    Empty,
    Digit,
    Symbol(char),
}

#[derive(Debug, Default)]
struct Schematic {
    grid: Vec<Vec<Cell>>,
    part_numbers: Vec<PartNumber>,
}

impl Schematic {
    fn from_input(input: &str) -> Self {
        let mut schematic = Schematic::default();
        for (y_pos, ln) in input.lines().enumerate() {
            let mut row: Vec<Cell> = Vec::default();
            let mut number_string: String = String::new();
            let mut part_number: PartNumber = PartNumber::default();
            for (x_pos, c) in ln.chars().enumerate() {
                match c {
                    c if c.is_ascii_digit() => {
                        row.push(Cell::Digit);
                        if number_string.is_empty() {
                            part_number.start = (x_pos as i32, y_pos as i32);
                        }
                        part_number.end = (x_pos as i32, y_pos as i32);
                        number_string.push(c);
                    }
                    '.' => {
                        row.push(Cell::Empty);
                        if !number_string.is_empty() {
                            part_number.value = number_string.parse().unwrap();
                            number_string.clear();
                            schematic.part_numbers.push(part_number.clone());
                        }
                    }
                    _ => {
                        row.push(Cell::Symbol(c));
                        if !number_string.is_empty() {
                            part_number.value = number_string.parse().unwrap();
                            number_string.clear();
                            schematic.part_numbers.push(part_number.clone());
                        }
                    }
                }
            }
            if !number_string.is_empty() {
                part_number.value = number_string.parse().unwrap();
                number_string.clear();
                schematic.part_numbers.push(part_number.clone());
            }
            schematic.grid.push(row);
        }
        schematic
    }
}

fn part1(input: &str) -> i32 {
    let schematic = Schematic::from_input(input);
    schematic
        .part_numbers
        .iter()
        .filter(|pn| pn.is_valid(&schematic.grid))
        .fold(0, |acc, pn| acc + pn.value)
}

fn part2(input: &str) -> i32 {
    let schematic = Schematic::from_input(input);
    let mut result = 0;
    for (y_pos, row) in schematic.grid.iter().enumerate() {
        for (x_pos, cell) in row.iter().enumerate() {
            if let Cell::Symbol('*') = cell {
                let neighbor_points = [
                    (x_pos as i32 - 1, y_pos as i32 - 1),
                    (x_pos as i32, y_pos as i32 - 1),
                    (x_pos as i32 + 1, y_pos as i32 - 1),
                    (x_pos as i32 - 1, y_pos as i32),
                    (x_pos as i32 + 1, y_pos as i32),
                    (x_pos as i32 - 1, y_pos as i32 + 1),
                    (x_pos as i32, y_pos as i32 + 1),
                    (x_pos as i32 + 1, y_pos as i32 + 1),
                ];

                let neighbors: Vec<_> = neighbor_points
                    .iter()
                    .map(|point| (get_cell(*point, &schematic.grid), *point))
                    .filter_map(|(cell, point)| {
                        if let Some(Cell::Digit) = cell {
                            return Some(point);
                        }
                        None
                    })
                    .collect();

                let mut adj_part_numbers: HashSet<&PartNumber> = HashSet::new();

                for part_number in &schematic.part_numbers {
                    for point in &neighbors {
                        if part_number.includes_point(point) {
                            adj_part_numbers.insert(part_number);
                        }
                    }
                }
                if adj_part_numbers.len() == 2 {
                    result += adj_part_numbers.iter().map(|pn| pn.value).product::<i32>();
                }
            }
        }
    }
    result
}

fn main() {
    let input = include_str!("../input/day3.txt");
    aoc2023::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day3.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 4361);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 467835);
    }
}

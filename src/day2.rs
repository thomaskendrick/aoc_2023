#[derive(Debug, Default)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug, Default)]
struct Game {
    id: i32,
    cubesets: Vec<CubeSet>,
    max_red: i32,
    max_green: i32,
    max_blue: i32,
}

impl Game {
    fn from_line(input: &str) -> Self {
        let mut game = Game::default();
        let (number_str, record_str) = input.split_once(':').unwrap();

        game.id = number_str
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        for cubeset_str in record_str.split(';') {
            let mut cubeset = CubeSet::default();
            for cube_str in cubeset_str.split(',') {
                let (count, colour) = cube_str.trim().split_once(' ').unwrap();
                match colour {
                    "red" => {
                        cubeset.red += count.parse::<i32>().unwrap();
                    }
                    "green" => {
                        cubeset.green += count.parse::<i32>().unwrap();
                    }
                    "blue" => {
                        cubeset.blue += count.parse::<i32>().unwrap();
                    }
                    _ => {
                        panic!("invalid colour")
                    }
                }
            }
            if cubeset.red > game.max_red {
                game.max_red = cubeset.red
            }
            if cubeset.green > game.max_green {
                game.max_green = cubeset.green
            }
            if cubeset.blue > game.max_blue {
                game.max_blue = cubeset.blue
            }
            game.cubesets.push(cubeset)
        }
        game
    }
    fn is_valid(&self, test: &CubeSet) -> bool {
        test.red >= self.max_red && test.green >= self.max_green && test.blue >= self.max_blue
    }
    fn min_cubes_power(&self) -> i32 {
        self.max_red * self.max_green * self.max_blue
    }
}

fn part1(input: &str) -> i32 {
    let test_set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .lines()
        .map(Game::from_line)
        .filter(|game| game.is_valid(&test_set))
        .fold(0, |acc, game| acc + game.id)
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(Game::from_line)
        .map(|game| game.min_cubes_power())
        .sum()
}

fn main() {
    let input = include_str!("../input/day2.txt");
    aoc2023::solve_puzzles(input, part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example/day2.txt");
    #[test]
    fn part_1_test() {
        assert_eq!(part1(EXAMPLE), 8);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part2(EXAMPLE), 2286);
    }
}

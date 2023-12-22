use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, Copy, Clone, Default)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut game = Game::default();

        let (heading, content) = input.split_once(": ").unwrap();
        game.id = heading.split_once(" ").unwrap().1.parse()?;

        for subset in content.split("; ") {
            for part in subset.split(", ") {
                let (num_cubes, color) = part.split_once(" ").unwrap();
                let num_cubes = num_cubes.parse::<u32>()?;
                match color {
                    "red" => {
                        if game.max_red < num_cubes {
                            game.max_red = num_cubes
                        }
                    }
                    "green" => {
                        if game.max_green < num_cubes {
                            game.max_green = num_cubes
                        }
                    }
                    "blue" => {
                        if game.max_blue < num_cubes {
                            game.max_blue = num_cubes
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        Ok(game)
    }
}

fn part1(input: &String) -> u32 {
    let required_game = Game {
        id: 0,
        max_red: 12,
        max_green: 13,
        max_blue: 14,
    };

    let mut sum = 0;

    for line in input.lines() {
        let game: Game = line.parse().unwrap();
        if !(game.max_red > required_game.max_red
            || game.max_green > required_game.max_green
            || game.max_blue > required_game.max_blue)
        {
            sum += game.id;
        }
    }

    sum
}

fn part2(input: &String) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let game: Game = line.parse().unwrap();
        sum += game.max_red * game.max_green * game.max_blue
    }

    sum
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

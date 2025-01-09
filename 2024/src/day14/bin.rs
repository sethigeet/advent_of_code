use std::str::FromStr;

use anyhow::Result;
use aoc2024::grid::Point;

#[derive(Debug)]
struct Robot {
    pos: Point,
    vel: Point<i64>,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (pos, vel) = s.split_once(" ").unwrap();

        let pos = &pos[2..];
        let pos = pos.split_once(",").unwrap();
        let pos = Point::new(pos.0.parse()?, pos.1.parse()?);

        let vel = &vel[2..];
        let vel = vel.split_once(",").unwrap();
        let vel = Point::new(vel.0.parse()?, vel.1.parse()?);

        Ok(Robot { pos, vel })
    }
}

// const GRID_SIZE: (usize, usize) = (11, 7);
const GRID_SIZE: (usize, usize) = (101, 103);

macro_rules! print_grid {
    ($grid:expr) => {
        for row in $grid.iter() {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    };
}

fn part1(input: &String) -> i64 {
    let mut robots: Vec<Robot> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut grid = vec![vec![0_i64; GRID_SIZE.0]; GRID_SIZE.1];

    for robot in robots.iter() {
        grid[robot.pos.y][robot.pos.x] += 1;
    }

    for _ in 0..100 {
        for i in 0..robots.len() {
            let robot = &mut robots[i];
            grid[robot.pos.y][robot.pos.x] -= 1;

            let mut new_pos = Point::new(
                (robot.pos.x as i64 + robot.vel.x) % GRID_SIZE.0 as i64,
                (robot.pos.y as i64 + robot.vel.y) % GRID_SIZE.1 as i64,
            );

            if new_pos.x < 0 {
                new_pos.x += GRID_SIZE.0 as i64;
            }
            if new_pos.y < 0 {
                new_pos.y += GRID_SIZE.1 as i64;
            }

            let new_pos = Point::new(new_pos.x as usize, new_pos.y as usize);
            robot.pos = new_pos;
            grid[robot.pos.y][robot.pos.x] += 1;
        }
    }

    // Set the center column and row to 0
    for i in 0..GRID_SIZE.1 {
        grid[i][GRID_SIZE.0 / 2] = 0;
    }
    for j in 0..GRID_SIZE.0 {
        grid[GRID_SIZE.1 / 2][j] = 0;
    }

    let (top, bottom) = grid.split_at(GRID_SIZE.1 / 2);
    let top_res =
        top.iter()
            .map(|row| row.split_at(GRID_SIZE.0 / 2))
            .fold((0, 0), |init, (left, right)| {
                (
                    init.0 + left.iter().sum::<i64>(),
                    init.1 + right.iter().sum::<i64>(),
                )
            });

    let bottom_res = bottom.iter().map(|row| row.split_at(GRID_SIZE.0 / 2)).fold(
        (0, 0),
        |init, (left, right)| {
            (
                init.0 + left.iter().sum::<i64>(),
                init.1 + right.iter().sum::<i64>(),
            )
        },
    );

    top_res.0 * bottom_res.0 * top_res.1 * bottom_res.1
}

fn part2(input: &String) -> usize {
    let mut robots: Vec<Robot> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut grid = vec![vec![0_i64; GRID_SIZE.0]; GRID_SIZE.1];

    for robot in robots.iter() {
        grid[robot.pos.y][robot.pos.x] += 1;
    }

    let mut seconds = 0;
    loop {
        seconds += 1;
        for i in 0..robots.len() {
            let robot = &mut robots[i];
            grid[robot.pos.y][robot.pos.x] -= 1;

            let mut new_pos = Point::new(
                (robot.pos.x as i64 + robot.vel.x) % GRID_SIZE.0 as i64,
                (robot.pos.y as i64 + robot.vel.y) % GRID_SIZE.1 as i64,
            );

            if new_pos.x < 0 {
                new_pos.x += GRID_SIZE.0 as i64;
            }
            if new_pos.y < 0 {
                new_pos.y += GRID_SIZE.1 as i64;
            }

            let new_pos = Point::new(new_pos.x as usize, new_pos.y as usize);
            robot.pos = new_pos;
            grid[robot.pos.y][robot.pos.x] += 1;
        }

        for row in grid.iter() {
            let mut count = 0;
            for cell in row {
                if *cell >= 1 {
                    count += 1;

                    if count >= 10 {
                        print_grid!(grid);
                        return seconds;
                    }
                } else {
                    count = 0;
                }
            }
        }

        for j in 0..grid[0].len() {
            let mut count = 0;
            for i in 0..grid.len() {
                let cell = grid[i][j];
                if cell >= 1 {
                    count += 1;

                    if count >= 10 {
                        print_grid!(grid);
                        return i;
                    }
                } else {
                    count = 0;
                }
            }
        }
    }
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

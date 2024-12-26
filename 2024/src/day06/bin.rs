use std::{collections::HashSet, str::FromStr};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CellType {
    Empty,
    Obstacle,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    r#type: CellType,
    visited: bool,
}

impl FromStr for Cell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Cell {
            r#type: match s {
                "#" => CellType::Obstacle,
                _ => CellType::Empty,
            },
            visited: false,
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Guard {
    location: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn rotate_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn at_edge(&self, grid: &Vec<Vec<Cell>>) -> bool {
        let (i, j) = self.location;
        match self.direction {
            Direction::Up => i == 0,
            Direction::Down => i == grid.len() - 1,
            Direction::Left => j == 0,
            Direction::Right => j == grid[0].len() - 1,
        }
    }

    fn move_forward(&mut self) {
        let (i, j) = self.location;
        match self.direction {
            Direction::Up => self.location = (i - 1, j),
            Direction::Down => self.location = (i + 1, j),
            Direction::Left => self.location = (i, j - 1),
            Direction::Right => self.location = (i, j + 1),
        }
    }
}

fn part1(input: &String) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut guard = Guard {
        location: (0, 0),
        direction: Direction::Up,
    };
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                guard.location = (i, j);
            }
        }
    }

    let mut grid: Vec<Vec<Cell>> = grid
        .iter()
        .map(|row| row.iter().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    // Mark the starting cell as visited
    grid[guard.location.0][guard.location.1].visited = true;

    while !guard.at_edge(&grid) {
        let next_location = match guard.direction {
            Direction::Up => (guard.location.0 - 1, guard.location.1),
            Direction::Down => (guard.location.0 + 1, guard.location.1),
            Direction::Left => (guard.location.0, guard.location.1 - 1),
            Direction::Right => (guard.location.0, guard.location.1 + 1),
        };

        if grid[next_location.0][next_location.1].r#type == CellType::Empty {
            guard.move_forward();
            grid[guard.location.0][guard.location.1].visited = true;
        } else {
            guard.rotate_right();
        }
    }

    grid.into_iter()
        .map(|row| row.into_iter().filter(|c| c.visited).count())
        .sum::<usize>() as i32
}

fn part2(input: &String) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut guard = Guard {
        location: (0, 0),
        direction: Direction::Up,
    };
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                guard.location = (i, j);
            }
        }
    }

    let mut grid: Vec<Vec<Cell>> = grid
        .iter()
        .map(|row| row.iter().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    let original_guard = guard.clone();

    // Find all the positions the guard visits
    grid[guard.location.0][guard.location.1].visited = true;
    while !guard.at_edge(&grid) {
        let next_location = match guard.direction {
            Direction::Up => (guard.location.0 - 1, guard.location.1),
            Direction::Down => (guard.location.0 + 1, guard.location.1),
            Direction::Left => (guard.location.0, guard.location.1 - 1),
            Direction::Right => (guard.location.0, guard.location.1 + 1),
        };

        if grid[next_location.0][next_location.1].r#type == CellType::Empty {
            guard.move_forward();
            grid[guard.location.0][guard.location.1].visited = true;
        } else {
            guard.rotate_right();
        }
    }

    let mut ans = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !grid[i][j].visited || (i, j) == original_guard.location {
                continue;
            }

            guard = original_guard.clone();
            let mut prev_guard_states = HashSet::new();
            prev_guard_states.insert(guard.clone());

            grid[i][j].r#type = CellType::Obstacle;

            while !guard.at_edge(&grid) {
                let next_location = match guard.direction {
                    Direction::Up => (guard.location.0 - 1, guard.location.1),
                    Direction::Down => (guard.location.0 + 1, guard.location.1),
                    Direction::Left => (guard.location.0, guard.location.1 - 1),
                    Direction::Right => (guard.location.0, guard.location.1 + 1),
                };

                if grid[next_location.0][next_location.1].r#type == CellType::Empty {
                    guard.move_forward();
                } else {
                    guard.rotate_right();
                }

                if prev_guard_states.contains(&guard) {
                    ans += 1;
                    break;
                }
                prev_guard_states.insert(guard.clone());
            }

            grid[i][j].r#type = CellType::Empty;
        }
    }

    ans
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

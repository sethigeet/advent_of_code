use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Mul, Sub},
};

use anyhow::Result;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Position {
    i: i64,
    j: i64,
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}

impl Mul<i64> for Position {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            i: self.i * rhs,
            j: self.j * rhs,
        }
    }
}

impl Position {
    fn new(input: (i64, i64)) -> Self {
        Self {
            i: input.0,
            j: input.1,
        }
    }

    fn is_valid(&self, grid_size: &(i64, i64)) -> bool {
        self.i >= 0 && self.i < grid_size.0 && self.j >= 0 && self.j < grid_size.1
    }
}

#[derive(Debug)]
struct Antenna {
    frequency: char,
    position: Position,
}

fn part1(input: &String) -> usize {
    let grid_size = (
        input.lines().count() as i64,
        input.lines().next().unwrap().chars().count() as i64,
    );

    let mut antennae = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }

            antennae.push(Antenna {
                frequency: char,
                position: Position::new((i as i64, j as i64)),
            });
        }
    }

    let mut antennae_groups: HashMap<char, Vec<Position>> = HashMap::new();
    for antenna in antennae.into_iter() {
        if let Some(v) = antennae_groups.get_mut(&antenna.frequency) {
            v.push(antenna.position);
        } else {
            antennae_groups.insert(antenna.frequency, vec![antenna.position]);
        }
    }

    let mut anti_nodes = HashSet::new();
    for (_, positions) in antennae_groups.iter() {
        for (a1, a2) in positions
            .iter()
            .enumerate()
            .flat_map(|(i, a1)| positions.iter().skip(i + 1).map(move |a2| (a1, a2)))
        {
            // Because of the way we get the pairs (a1, a2), we will always have a1 < a2
            let diff = *a2 - *a1;

            let anti_node = *a1 - diff;
            if anti_node.is_valid(&grid_size) {
                anti_nodes.insert(anti_node);
            }

            let anti_node = *a2 + diff;
            if anti_node.is_valid(&grid_size) {
                anti_nodes.insert(anti_node);
            }
        }
    }

    anti_nodes.len()
}

fn part2(input: &String) -> usize {
    let grid_size = (
        input.lines().count() as i64,
        input.lines().next().unwrap().chars().count() as i64,
    );

    let mut antennae = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }

            antennae.push(Antenna {
                frequency: char,
                position: Position::new((i as i64, j as i64)),
            });
        }
    }

    let mut antennae_groups: HashMap<char, Vec<Position>> = HashMap::new();
    for antenna in antennae.into_iter() {
        if let Some(v) = antennae_groups.get_mut(&antenna.frequency) {
            v.push(antenna.position);
        } else {
            antennae_groups.insert(antenna.frequency, vec![antenna.position]);
        }
    }

    let mut anti_nodes = HashSet::new();
    for (_, positions) in antennae_groups.iter() {
        for position in positions.iter() {
            anti_nodes.insert(*position);
        }

        for (a1, a2) in positions
            .iter()
            .enumerate()
            .flat_map(|(i, a1)| positions.iter().skip(i + 1).map(move |a2| (a1, a2)))
        {
            // Because of the way we get the pairs (a1, a2), we will always have a1 < a2
            let diff = *a2 - *a1;

            let mut n = 1;
            loop {
                let mut nodes_added = 0;

                let anti_node = *a1 - (diff * n);
                if anti_node.is_valid(&grid_size) {
                    anti_nodes.insert(anti_node);
                    nodes_added += 1;
                }

                let anti_node = *a2 + (diff * n);
                if anti_node.is_valid(&grid_size) {
                    anti_nodes.insert(anti_node);
                    nodes_added += 1;
                }

                if nodes_added == 0 {
                    break;
                }
                n += 1;
            }
        }
    }

    anti_nodes.len()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

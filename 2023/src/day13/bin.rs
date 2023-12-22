use core::panic;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
enum FloorType {
    Ash,
    Rock,
}

impl From<char> for FloorType {
    fn from(input: char) -> Self {
        match input {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => unreachable!(),
        }
    }
}

type Floor = Vec<Vec<FloorType>>;

#[derive(Debug, Clone)]
struct Pattern {
    floor: Floor,
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            floor: input
                .lines()
                .map(|line| line.chars().map(|c| c.into()).collect())
                .collect(),
        })
    }
}

impl Pattern {
    fn get_mirror_col_err(&self, id: usize) -> usize {
        let mut side1_ids = 0..=id;
        let mut side2_ids = id + 1..self.floor[0].len();

        let side1_len = id + 1;
        let side2_len = (self.floor[0].len() - 1) - id;

        if side1_len > side2_len {
            side1_ids = id - (side2_len - 1)..=id;
        } else if side1_len < side2_len {
            side2_ids = id + 1..id + 1 + side1_len + 1;
        }

        let mut err_amt = 0;
        for (side1_id, side2_id) in side1_ids.rev().zip(side2_ids) {
            for i in 0..self.floor.len() {
                if self.floor[i][side1_id] != self.floor[i][side2_id] {
                    err_amt += 1;
                }
            }
        }

        err_amt
    }

    fn get_mirror_row_err(&self, id: usize) -> usize {
        let mut side1_ids = 0..=id;
        let mut side2_ids = id + 1..self.floor.len();

        let side1_len = id + 1;
        let side2_len = (self.floor.len() - 1) - id;

        if side1_len > side2_len {
            side1_ids = id - (side2_len - 1)..=id;
        } else if side1_len < side2_len {
            side2_ids = id + 1..id + 1 + side1_len + 1;
        }

        let mut err_amt = 0;
        for (side1_id, side2_id) in side1_ids.rev().zip(side2_ids) {
            for i in 0..self.floor[0].len() {
                if self.floor[side1_id][i] != self.floor[side2_id][i] {
                    err_amt += 1;
                }
            }
        }

        err_amt
    }

    fn find_mirror_col(&self, err_amt: usize) -> Option<usize> {
        for i in 0..self.floor[0].len() - 1 {
            if self.get_mirror_col_err(i) == err_amt {
                return Some(i);
            }
        }

        None
    }

    fn find_mirror_row(&self, err_amt: usize) -> Option<usize> {
        for i in 0..self.floor.len() - 1 {
            if self.get_mirror_row_err(i) == err_amt {
                return Some(i);
            }
        }

        None
    }
}

fn solve(patterns: &Vec<Pattern>, err_amt: usize) -> usize {
    patterns
        .iter()
        .map(|pattern| {
            if let Some(idx) = pattern.find_mirror_col(err_amt) {
                idx + 1 // Add 1 as the problem assumes that indexes start at 1
            } else if let Some(idx) = pattern.find_mirror_row(err_amt) {
                (idx + 1) * 100 // Add 1 as the problem assumes that indexes start at 1
            } else {
                panic!("Unable to find mirror in either row or column.");
            }
        })
        .sum::<usize>()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let patterns: Vec<Pattern> = input
        .split("\n\n")
        .map(|pattern| pattern.parse().unwrap())
        .collect();

    println!("Part 1: {}", solve(&patterns, 0));
    println!("Part 2: {}", solve(&patterns, 1));

    Ok(())
}

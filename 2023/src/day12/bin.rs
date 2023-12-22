use std::str::FromStr;

use anyhow::Result;
use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

impl From<char> for Spring {
    fn from(input: char) -> Self {
        use Spring::*;
        match input {
            '#' => Damaged,
            '.' => Operational,
            '?' => Unknown,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    damaged_groups: Vec<u32>,
}

impl FromStr for Row {
    type Err = anyhow::Error;

    // fn from_str(input: &str) -> Result<Self, Self::Err> {
    //     let (springs, damaged_groups) = input.split_once(" ").unwrap();
    //     Ok(Self {
    //         springs: springs.chars().map(|c| c.into()).collect(),
    //         damaged_groups: damaged_groups
    //             .split(",")
    //             .map(|c| c.parse().unwrap())
    //             .collect(),
    //     })
    // }

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (springs, damaged_groups) = input.split_once(" ").unwrap();
        let springs: Vec<Spring> = springs.chars().map(|c| c.into()).collect();
        let damaged_groups: Vec<u32> = damaged_groups
            .split(",")
            .map(|c| c.parse().unwrap())
            .collect();

        Ok(Self {
            springs: repeat_n(springs, 5)
                .intersperse(vec![Spring::Unknown])
                .flatten()
                .collect(),
            damaged_groups: repeat_n(damaged_groups, 5).flatten().collect(),
        })
    }
}

impl Row {
    fn get_num_unknowns(&self) -> usize {
        self.springs
            .iter()
            .filter(|spring| spring == &&Spring::Unknown)
            .count()
    }

    fn is_valid(&self, new_seq: &Vec<Spring>) -> bool {
        let mut groups = Vec::with_capacity(self.damaged_groups.len());
        for (is_damaged, springs) in &new_seq
            .iter()
            .group_by(|spring| spring == &&Spring::Damaged)
        {
            if is_damaged {
                groups.push(springs.count() as u32)
            }
        }

        groups == self.damaged_groups
    }
}

fn solve(input: &str) -> u32 {
    let rows: Vec<Row> = input.lines().map(|line| line.parse().unwrap()).collect();

    rows.iter().fold(0, |acc, row| {
        let num_valid_replacements = repeat_n(
            [Spring::Damaged, Spring::Operational],
            row.get_num_unknowns(),
        )
        .multi_cartesian_product()
        .fold(0, |acc, replacement| {
            let mut new_seq = row.springs.clone();
            let mut i = 0;
            for spring in new_seq.iter_mut() {
                if spring == &mut Spring::Unknown {
                    replacement[i].clone_into(spring);
                    i += 1;
                }
            }

            if row.is_valid(&new_seq) {
                return acc + 1;
            } else {
                return acc;
            }
        });

        acc + num_valid_replacements
    }) as u32
}

fn main() -> Result<()> {
    let input = include_str!("./sample_input.txt").to_string();
    // let input = include_str!("./input.txt").to_string();

    println!("Part 1/2: {}", solve(&input));

    Ok(())
}

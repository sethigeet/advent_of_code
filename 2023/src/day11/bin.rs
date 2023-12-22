use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Point {
    Space,
    Galaxy,
}

impl From<char> for Point {
    fn from(input: char) -> Self {
        match input {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => unreachable!(),
        }
    }
}

type Image = Vec<Vec<Point>>;

fn solve(img: &Image, mul_factor: i64) -> u64 {
    let mut empty_row_ids = vec![];
    for j in 0..img.len() {
        if !&img[j].iter().any(|p| p == &Point::Galaxy) {
            empty_row_ids.push(j);
        }
    }

    let mut empty_col_ids = vec![];
    for i in 0..img[0].len() {
        if !img.iter().map(|row| row[i]).any(|p| p == Point::Galaxy) {
            empty_col_ids.push(i);
        }
    }

    let galaxy_coords: Vec<(usize, usize)> = img
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, point)| {
                if point == &Point::Galaxy {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    let mut sum = 0;
    for (g1, g2) in galaxy_coords.iter().tuple_combinations() {
        let x_min = g1.0.min(g2.0);
        let x_max = g1.0.max(g2.0);
        let y_min = g1.1.min(g2.1);
        let y_max = g1.1.max(g2.1);

        let x_dist = (g1.0 as i64 - g2.0 as i64).abs()
            + empty_col_ids
                .iter()
                .filter(|id| **id > x_min && **id < x_max)
                .count() as i64
                * (mul_factor - 1);
        let y_dist = (g1.1 as i64 - g2.1 as i64).abs()
            + empty_row_ids
                .iter()
                .filter(|id| **id > y_min && **id < y_max)
                .count() as i64
                * (mul_factor - 1);
        sum += x_dist + y_dist;
    }

    sum as u64
}

fn part1(img: &Image) -> u64 {
    solve(img, 2)
}

fn part2(img: &Image) -> u64 {
    solve(img, 1_000_000)
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let img: Image = input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();

    println!("Part 1: {}", part1(&img));
    println!("Part 2: {}", part2(&img));

    Ok(())
}

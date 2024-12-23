use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct HailStone {
    pos: (f64, f64, f64),
    vel: (f64, f64, f64),
}

impl FromStr for HailStone {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = input.split_once(" @ ").unwrap();
        let pos: Vec<_> = pos
            .split(", ")
            .map(|num| num.trim().parse().unwrap())
            .collect();
        let vel: Vec<_> = vel
            .split(", ")
            .map(|num| num.trim().parse().unwrap())
            .collect();
        Ok(Self {
            pos: (pos[0], pos[1], pos[2]),
            vel: (vel[0], vel[1], vel[2]),
        })
    }
}

impl HailStone {
    fn get_pos_at_time(&self, time: f64) -> (f64, f64, f64) {
        (
            self.pos.0 + self.vel.0 * time,
            self.pos.1 + self.vel.1 * time,
            self.pos.2 + self.vel.2 * time,
        )
    }

    fn intersects_at_xy(&self, other: &HailStone) -> Option<(f64, f64)> {
        use ndarray::prelude::*;
        use ndarray_linalg::Solve;

        let a: Array2<f64> = array![[self.vel.0, -other.vel.0], [self.vel.1, -other.vel.1]];
        let b: Array1<f64> = array![other.pos.0 - self.pos.0, other.pos.1 - self.pos.1];
        let x = a.solve_into(b);
        if x.is_err() {
            return None;
        }
        let x = x.unwrap();
        Some((x[0], x[1]))
    }
}

fn part1(input: &str) -> usize {
    let hailstones: Vec<HailStone> = input.lines().map(|line| line.parse().unwrap()).collect();
    let bounds = 200000000000000f64..=400000000000000f64;

    hailstones
        .iter()
        .tuple_combinations()
        .filter(|(ha, hb)| {
            let res = ha.intersects_at_xy(hb);
            if res.is_none() {
                return false;
            }
            let res = res.unwrap();
            let pos = ha.get_pos_at_time(res.0);
            res.0 > 0.0 && res.1 > 0.0 && bounds.contains(&pos.0) && bounds.contains(&pos.1)
        })
        .count()
}

// fn part2(map: &Map, dimensions: &(f64, f64), start_pos: &(f64, f64)) -> usize {
// }

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&map, &dimensions, start_pos));

    Ok(())
}

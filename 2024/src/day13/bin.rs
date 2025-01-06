use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct Machine {
    move_by_a: (i64, i64),
    move_by_b: (i64, i64),
    prize_location: (i64, i64),
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut res = s.lines().map(|line| {
            let loc = line.split_once(": ").unwrap().1.split_once(", ").unwrap();
            (loc.0[2..].parse().unwrap(), loc.1[2..].parse().unwrap())
        });

        Ok(Machine {
            move_by_a: res.next().unwrap(),
            move_by_b: res.next().unwrap(),
            prize_location: res.next().unwrap(),
        })
    }
}

const COST: (i64, i64) = (3, 1);

impl Machine {
    fn get_num_moves_required_1(&self) -> Option<(i64, i64)> {
        let mut moves = (0, 0);

        for i in 0..=100 {
            for j in 0..=100 {
                let new_location = (
                    i * self.move_by_a.0 + j * self.move_by_b.0,
                    i * self.move_by_a.1 + j * self.move_by_b.1,
                );
                if new_location == self.prize_location {
                    if moves == (0, 0) {
                        moves = (i, j);
                    } else if i * COST.0 + j * COST.1 < moves.0 * 3 + moves.1 {
                        moves = (i, j);
                    }
                }
            }
        }

        if moves == (0, 0) {
            None
        } else {
            Some(moves)
        }
    }

    fn get_num_moves_required_2(&self) -> Option<(i64, i64)> {
        let det =
            (self.move_by_a.0 * self.move_by_b.1 - self.move_by_a.1 * self.move_by_b.0) as f64;
        if det == 0.0 {
            return None;
        }

        let moves = (
            ((self.prize_location.0 * self.move_by_b.1 - self.prize_location.1 * self.move_by_b.0)
                as f64
                / det),
            ((self.move_by_a.0 * self.prize_location.1 - self.move_by_a.1 * self.prize_location.0)
                as f64
                / det),
        );
        if moves.0 < 0.0
            || moves.1 < 0.0
            || moves.0.trunc() != moves.0
            || moves.1.trunc() != moves.1
        {
            return None;
        }

        Some((moves.0 as i64, moves.1 as i64))
    }
}

fn part1(input: &String) -> i64 {
    let machines: Vec<Machine> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();

    machines
        .iter()
        .filter_map(|f| f.get_num_moves_required_1())
        .map(|(a, b)| a * COST.0 + b * COST.1)
        .sum()
}

fn part2(input: &String) -> i64 {
    let mut machines: Vec<Machine> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
    machines.iter_mut().for_each(|machine| {
        machine.prize_location.0 += 10000000000000;
        machine.prize_location.1 += 10000000000000;
    });

    machines
        .iter()
        .filter_map(|f| f.get_num_moves_required_2())
        .map(|(a, b)| a * COST.0 + b * COST.1)
        .sum()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct Equation {
    result: i128,
    operands: Vec<i128>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_once(": ").unwrap();
        let result = parts.0.parse::<i128>().unwrap();
        let operands = parts
            .1
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i128>>();

        Ok(Equation { result, operands })
    }
}

impl Equation {
    fn is_valid_1(&self) -> bool {
        if self.operands.len() == 1 {
            return self.result == self.operands[0];
        }

        // If last operand was a addition
        let mut sub_eqn = Equation {
            result: self.result - self.operands[self.operands.len() - 1],
            operands: self.operands[..self.operands.len() - 1].to_vec(),
        };
        if sub_eqn.is_valid_1() {
            return true;
        }

        // If last operand was a multiplication
        if self.result % self.operands[self.operands.len() - 1] != 0 {
            return false;
        }
        sub_eqn.result = self.result / self.operands[self.operands.len() - 1];
        if sub_eqn.is_valid_1() {
            return true;
        }

        false
    }

    fn is_valid_2(&self) -> bool {
        if self.operands.len() == 1 {
            return self.result == self.operands[0];
        }

        // If last operation was addition
        let mut sub_eqn = Equation {
            result: self.result - self.operands[self.operands.len() - 1],
            operands: self.operands[..self.operands.len() - 1].to_vec(),
        };
        if sub_eqn.is_valid_2() {
            return true;
        }

        // If last operation was multiplication
        if self.result % self.operands[self.operands.len() - 1] == 0 {
            sub_eqn.result = self.result / self.operands[self.operands.len() - 1];
            if sub_eqn.is_valid_2() {
                return true;
            }
        }

        // If last operation was concatenation
        let denominator = i128::pow(
            10,
            (self.operands[self.operands.len() - 1] as f64)
                .log10()
                .floor() as u32
                + 1,
        );
        if self.result % denominator == self.operands[self.operands.len() - 1] {
            sub_eqn.result = self.result / denominator;
            if sub_eqn.is_valid_2() {
                return true;
            }
        }

        false
    }
}

fn part1(input: &String) -> i128 {
    let equations: Vec<Equation> = input.split("\n").map(|x| x.parse().unwrap()).collect();

    equations
        .into_iter()
        .filter(|eqn| eqn.is_valid_1())
        .map(|eqn| eqn.result)
        .sum()
}

fn part2(input: &String) -> i128 {
    let equations: Vec<Equation> = input.split("\n").map(|x| x.parse().unwrap()).collect();

    equations
        .into_iter()
        .filter(|eqn| eqn.is_valid_2())
        .map(|eqn| eqn.result)
        .sum()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct Rule {
    x: i32,
    y: i32,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s.split_once("|").unwrap();
        Ok(Rule {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

fn part1(input: &String) -> i32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules.lines().map(|line| line.parse().unwrap()).collect();
    let updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    updates
        .into_iter()
        .filter(|update| {
            rules.iter().all(|rule| {
                let id_x = update.iter().position(|&x| x == rule.x);
                let id_y = update.iter().position(|&x| x == rule.y);

                if id_x.is_none() || id_y.is_none() {
                    return true;
                }

                id_x.unwrap() < id_y.unwrap()
            })
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(input: &String) -> i32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules.lines().map(|line| line.parse().unwrap()).collect();
    let updates: Vec<Vec<i32>> = updates
        .lines()
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    updates
        .into_iter()
        .filter(|update| {
            !rules.iter().all(|rule| {
                let id_x = update.iter().position(|&x| x == rule.x);
                let id_y = update.iter().position(|&x| x == rule.y);

                if id_x.is_none() || id_y.is_none() {
                    return true;
                }

                id_x.unwrap() < id_y.unwrap()
            })
        })
        .map(|mut update| {
            let mut is_valid = false;
            while !is_valid {
                rules.iter().for_each(|rule| {
                    let id_x = update.iter().position(|&x| x == rule.x);
                    let id_y = update.iter().position(|&x| x == rule.y);

                    if id_x.is_some() && id_y.is_some() && id_x.unwrap() > id_y.unwrap() {
                        let temp = update[id_x.unwrap()];
                        update[id_x.unwrap()] = update[id_y.unwrap()];
                        update[id_y.unwrap()] = temp;
                    }
                });

                is_valid = rules.iter().all(|rule| {
                    let id_x = update.iter().position(|&x| x == rule.x);
                    let id_y = update.iter().position(|&x| x == rule.y);

                    if id_x.is_none() || id_y.is_none() {
                        return true;
                    }

                    id_x.unwrap() < id_y.unwrap()
                })
            }

            update
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

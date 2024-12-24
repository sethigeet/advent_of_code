use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<i32>,
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let levels: Vec<i32> = s
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        Ok(Report { levels })
    }
}

fn part1(reports: &Vec<Report>) -> i32 {
    reports
        .iter()
        .filter(|r| {
            let mut iter = r.levels.iter();
            let first = iter.next().unwrap();
            let mut prev = iter.next().unwrap();

            if !(1..=3).contains(&first.abs_diff(*prev)) {
                return false;
            }

            let increasing = first < prev;

            for level in iter {
                if increasing && level < prev {
                    return false;
                } else if !increasing && level > prev {
                    return false;
                }

                if !(1..=3).contains(&level.abs_diff(*prev)) {
                    return false;
                }

                prev = level;
            }

            true
        })
        .count() as i32
}

fn part2(reports: &Vec<Report>) -> i32 {
    let mut ans = 0;

    for r in reports {
        let mut new_reports = Vec::new();
        new_reports.push(r.clone());

        let mut perms = Vec::new();
        for i in 0..r.levels.len() {
            let mut perm = Vec::new();
            for j in 0..r.levels.len() {
                if i != j {
                    perm.push(r.levels[j]);
                }
            }
            perms.push(perm);
        }

        for perm in perms {
            new_reports.push(Report { levels: perm });
        }
        if part1(&new_reports) > 0 {
            ans += 1;
        }
    }

    ans
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let reports = input
        .lines()
        .map(|line| line.parse::<Report>().unwrap())
        .collect::<Vec<Report>>();

    println!("Part 1: {}", part1(&reports));
    println!("Part 2: {}", part2(&reports));

    Ok(())
}

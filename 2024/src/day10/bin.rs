use std::collections::HashSet;

use anyhow::Result;

fn get_trailends(
    topology: &Vec<Vec<i64>>,
    start: (usize, usize),
    trail_ends: &mut HashSet<(usize, usize)>,
) {
    if topology[start.0][start.1] == 9 {
        trail_ends.insert(start);
        return;
    }

    if start.0 > 0 && topology[start.0 - 1][start.1] - topology[start.0][start.1] == 1 {
        get_trailends(topology, (start.0 - 1, start.1), trail_ends);
    }
    if start.0 < topology.len() - 1
        && topology[start.0 + 1][start.1] - topology[start.0][start.1] == 1
    {
        get_trailends(topology, (start.0 + 1, start.1), trail_ends);
    }
    if start.1 > 0 && topology[start.0][start.1 - 1] - topology[start.0][start.1] == 1 {
        get_trailends(topology, (start.0, start.1 - 1), trail_ends);
    }
    if start.1 < topology[0].len() - 1
        && topology[start.0][start.1 + 1] - topology[start.0][start.1] == 1
    {
        get_trailends(topology, (start.0, start.1 + 1), trail_ends);
    }
}

fn part1(input: &String) -> usize {
    let topology: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    let mut score = 0;
    for (i, row) in topology.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                let mut trail_ends = HashSet::new();
                get_trailends(&topology, (i, j), &mut trail_ends);
                score += trail_ends.len();
            }
        }
    }

    score
}

fn get_trailhead_rating(topology: &Vec<Vec<i64>>, start: (usize, usize)) -> i64 {
    if topology[start.0][start.1] == 9 {
        return 1;
    }

    let mut score = 0;
    if start.0 > 0 && topology[start.0 - 1][start.1] - topology[start.0][start.1] == 1 {
        score += get_trailhead_rating(topology, (start.0 - 1, start.1));
    }
    if start.0 < topology.len() - 1
        && topology[start.0 + 1][start.1] - topology[start.0][start.1] == 1
    {
        score += get_trailhead_rating(topology, (start.0 + 1, start.1));
    }
    if start.1 > 0 && topology[start.0][start.1 - 1] - topology[start.0][start.1] == 1 {
        score += get_trailhead_rating(topology, (start.0, start.1 - 1));
    }
    if start.1 < topology[0].len() - 1
        && topology[start.0][start.1 + 1] - topology[start.0][start.1] == 1
    {
        score += get_trailhead_rating(topology, (start.0, start.1 + 1));
    }
    score
}

fn part2(input: &String) -> i64 {
    let topology: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    let mut score = 0;
    for (i, row) in topology.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                score += get_trailhead_rating(&topology, (i, j));
            }
        }
    }

    score
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

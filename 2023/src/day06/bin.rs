use anyhow::Result;

fn part1(input: &str) -> u32 {
    let (durations, distances) = input.split_once("\n").unwrap();

    let mut durations = durations.split_ascii_whitespace();
    durations.next();
    let durations: Vec<u32> = durations.map(|d| d.parse().unwrap()).collect();

    let mut distances = distances.split_ascii_whitespace();
    distances.next();
    let distances: Vec<u32> = distances.map(|d| d.parse().unwrap()).collect();

    durations
        .iter()
        .zip(distances)
        .fold(1, |acc, (duration, distance)| {
            acc * (1..*duration).fold(0, |acc, time| {
                if time * (duration - time) > distance {
                    acc + 1
                } else {
                    acc
                }
            })
        })
}

fn part2(input: &str) -> u64 {
    let nums: Vec<u64> = input
        .replace(" ", "")
        .lines()
        .map(|line| line.split_once(":").unwrap().1.parse().unwrap())
        .collect();
    let duration = nums[0];
    let distance = nums[1];

    (1..duration).fold(0, |acc, time| {
        if time * (duration - time) > distance {
            acc + 1
        } else {
            acc
        }
    })
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

use anyhow::Result;

fn extrapolate_ahead(nums: &Vec<i32>) -> i32 {
    if nums.iter().all(|n| *n == 0) {
        return 0;
    }

    let diffs = nums.windows(2).into_iter().map(|w| w[1] - w[0]).collect();

    nums[nums.len() - 1] + extrapolate_ahead(&diffs)
}

fn extrapolate_behind(nums: &Vec<i32>) -> i32 {
    if nums.iter().all(|n| *n == 0) {
        return 0;
    }

    let diffs = nums.windows(2).into_iter().map(|w| w[1] - w[0]).collect();

    nums[0] - extrapolate_behind(&diffs)
}

fn part1(readings: &Vec<Vec<i32>>) -> i32 {
    readings.iter().map(|r| extrapolate_ahead(r)).sum()
}

fn part2(readings: &Vec<Vec<i32>>) -> i32 {
    readings.iter().map(|r| extrapolate_behind(r)).sum()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let readings: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    println!("Part 1: {}", part1(&readings));
    println!("Part 2: {}", part2(&readings));

    Ok(())
}

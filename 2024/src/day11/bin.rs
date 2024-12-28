use anyhow::Result;
use rayon::prelude::*;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

static CACHE: Lazy<Mutex<HashMap<(usize, usize), usize>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn get_num_stones(stone: usize, num_iters: usize) -> usize {
    let key = (stone, num_iters);

    if let Some(&result) = CACHE.lock().unwrap().get(&key) {
        return result;
    }

    let result = if num_iters == 0 {
        1
    } else if stone == 0 {
        get_num_stones(1, num_iters - 1)
    } else if ((stone as f64).log10().floor() + 1.0) as usize % 2 == 0 {
        let num_digits = (stone as f64).log10().floor() + 1.0;
        let left = stone / 10usize.pow((num_digits / 2.0) as u32);
        let right = stone % 10usize.pow((num_digits / 2.0) as u32);

        Vec::from([left, right])
            .into_par_iter()
            .map(|x| get_num_stones(x, num_iters - 1))
            .sum()
    } else {
        get_num_stones(stone * 2024, num_iters - 1)
    };

    CACHE.lock().unwrap().insert(key, result);
    result
}

fn solve(input: &String, num_iters: usize) -> usize {
    let stones: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    stones
        .into_par_iter()
        .map(|stone| get_num_stones(stone, num_iters))
        .sum()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", solve(&input, 25));
    println!("Part 2: {}", solve(&input, 75));

    Ok(())
}

use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let mut joltages: Vec<u32> = std::fs::read_to_string("./data/inputs/10.txt")?
        .lines()
        .map(|line| line.parse().expect("line is not a number"))
        .sorted()
        .collect();

    let to_insert: Vec<u32> = vec![0];
    joltages.splice(0..0, to_insert);
    let to_insert: Vec<u32> = vec![joltages.iter().max().unwrap() + 3];
    joltages.splice(joltages.len()..joltages.len(), to_insert);

    let mut diff_1: u32 = 0;
    let mut diff_3: u32 = 0;
    let mut i: usize = 0;
    while i < joltages.len() - 1 {
        let curr = joltages[i];
        let next = joltages[i + 1];
        if next - curr == 1 {
            diff_1 += 1;
        } else if next - curr == 3 {
            diff_3 += 1;
        }

        i += 1;
    }

    println!("Part 1 -> {}", diff_1 * diff_3);

    let mut cache: HashMap<usize, u128> = HashMap::new();
    println!("Part 2 -> {}", get_num_ways(&joltages, 0, &mut cache));

    Ok(())
}

fn get_num_ways(joltages: &Vec<u32>, pos: usize, cache: &mut HashMap<usize, u128>) -> u128 {
    if pos == (joltages.len() - 1) {
        return 1;
    }

    if let Some(val) = cache.get(&pos) {
        return *val;
    }

    let mut total: u128 = 0;
    for i in pos + 1..joltages.len() {
        if joltages[i] - joltages[pos] <= 3 {
            total += get_num_ways(joltages, i, cache)
        }
    }

    cache.insert(pos, total);

    return total;
}

use std::collections::HashMap;

use anyhow::Result;

fn part1(banks: &Vec<Vec<u64>>) -> u64 {
    let mut ans = 0;
    for bank in banks {
        let mut highest_idx = 0;
        let mut highest_num = bank[0];
        for (idx, num) in bank.iter().enumerate() {
            if *num > highest_num {
                highest_num = *num;
                highest_idx = idx;
            }
        }

        if highest_idx == bank.len() - 1 {
            let mut second_highest_num = bank[0];
            for i in 0..bank.len() - 1 {
                if bank[i] > second_highest_num {
                    second_highest_num = bank[i];
                }
            }
            ans += second_highest_num * 10 + highest_num;
        } else {
            let second_highest_num = bank[highest_idx + 1..].iter().max().unwrap();
            ans += highest_num * 10 + second_highest_num;
        }
    }
    ans
}

fn get_max_joltage_for_bank(
    bank: &[u64],
    num_batteries_to_use: u64,
    cache: &mut HashMap<(usize, u64), u64>,
    original_bank_size: usize,
) -> u64 {
    if num_batteries_to_use == 0 {
        return 0;
    }
    let cache_key = (original_bank_size - bank.len(), num_batteries_to_use);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }
    if bank.len() == num_batteries_to_use as usize {
        let mut total_joltage = 0;
        for battery in bank {
            total_joltage *= 10;
            total_joltage += *battery;
        }
        return total_joltage;
    }

    let max_joltage =
        get_max_joltage_for_bank(&bank[1..], num_batteries_to_use, cache, original_bank_size).max(
            bank[0] * 10_u64.pow((num_batteries_to_use - 1) as u32)
                + get_max_joltage_for_bank(
                    &bank[1..],
                    num_batteries_to_use - 1,
                    cache,
                    original_bank_size,
                ),
        );
    cache.insert(cache_key, max_joltage);
    return max_joltage;
}

fn part2(banks: &Vec<Vec<u64>>) -> u64 {
    let mut ans = 0;
    for bank in banks {
        let mut cache: HashMap<(usize, u64), u64> = HashMap::new();
        ans += get_max_joltage_for_bank(&bank[..], 12, &mut cache, bank.len());
    }
    ans
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let banks: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap().into())
                .collect()
        })
        .collect();

    println!("Part 1: {}", part1(&banks));
    println!("Part 2: {}", part2(&banks));

    Ok(())
}

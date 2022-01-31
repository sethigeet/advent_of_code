use std::collections::HashMap;

use anyhow::Result;

fn get_nth_num(nums: &[usize], n: usize) -> usize {
    let mut counts: HashMap<usize, Vec<usize>> = HashMap::new();

    nums.iter().enumerate().for_each(|(i, num)| {
        counts.insert(*num, vec![(i + 1)]);
    });

    let mut prev_num = nums[nums.len() - 1];
    for i in nums.len()..n {
        if i == 1 {
            prev_num = 0;
            continue;
        }

        if let Some(val) = counts.get_mut(&prev_num) {
            prev_num = i - val[val.len() - 1];
            val.push(i);
            continue;
        }

        counts.insert(prev_num, vec![i]);
        prev_num = 0;
    }

    prev_num
}

fn main() -> Result<()> {
    let nums: Vec<usize> = std::fs::read_to_string("./data/inputs/15.txt")?
        .split(',')
        .map(|line| line.parse().expect("line is not a number"))
        .collect();

    println!("Part 1 -> {}", get_nth_num(&nums, 2020));
    println!("Part 2 -> {}", get_nth_num(&nums, 30_000_000));

    Ok(())
}

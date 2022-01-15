use anyhow::Result;
use itertools::Itertools;

const PREAMBLE: usize = 25;

fn comb_is_valid(allowed_nums: &[u64], curr: &u64) -> bool {
    for comb in allowed_nums.iter().combinations(2) {
        if (comb[0] + comb[1]) == *curr {
            return true;
        }
    }

    false
}

fn main() -> Result<()> {
    let nums: Vec<u64> = std::fs::read_to_string("./data/inputs/9.txt")?
        .lines()
        .map(|line| line.parse().expect("line is not a number"))
        .collect();

    let mut i = PREAMBLE;
    let mut part1: &u64 = &0;
    while i < nums.len() {
        let allowed_nums = &nums[i - PREAMBLE..i];
        let curr = &nums[i];

        if !comb_is_valid(allowed_nums, curr) {
            part1 = curr;
            break;
        }

        i += 1;
    }

    println!("Part 1 -> {}", part1);

    let mut len: usize = 2;
    i = 2;
    'len_loop: while len < nums.len() {
        while i < nums.len() {
            let allowed_nums = &nums[i - len..i];
            if allowed_nums.iter().sum::<u64>() == *part1 {
                let ans = allowed_nums.iter().max().unwrap() + allowed_nums.iter().min().unwrap();
                println!("Part 2 -> {}", ans);
                break 'len_loop;
            }

            i += 1;
        }

        len += 1;
        i = len;
    }

    Ok(())
}

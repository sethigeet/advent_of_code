use anyhow::Result;
use itertools::Itertools;

// NOTE: More lengthy implementation(though these are MUCH FASTERRRR)
// fn part1(nums: &Vec<u32>) -> u32 {
//     for (i, num1) in nums.into_iter().enumerate() {
//         for (j, num2) in nums.into_iter().enumerate() {
//             if i == j {
//                 continue;
//             }

//             if num1 + num2 == 2020 {
//                 return num1 * num2;
//             }
//         }
//     }

//     0
// }
// fn part2(nums: &Vec<u32>) -> u32 {
//     for (i, num1) in nums.into_iter().enumerate() {
//         for (j, num2) in nums.into_iter().enumerate() {
//             for (k, num3) in nums.into_iter().enumerate() {
//                 if i == j || j == k || i == k {
//                     continue;
//                 }

//                 if num1 + num2 + num3 == 2020 {
//                     return num1 * num2 * num3;
//                 }
//             }
//         }
//     }

//     0
// }

// NOTE: A much cleaner implementation(though this is MUCH SLOWERRR)
fn solve(nums: Vec<u32>, n: usize) -> u32 {
    for comb in nums.into_iter().combinations(n) {
        if comb.iter().sum::<u32>() == 2020 {
            return comb.iter().product();
        }
    }

    0
}

fn main() -> Result<()> {
    let nums: Vec<u32> = std::fs::read_to_string("./data/inputs/1.txt")?
        .lines()
        .map(|line| line.parse().expect("line is not a number"))
        .collect();

    println!("Part 1: {}", solve(nums.clone(), 2));
    println!("Part 2: {}", solve(nums.clone(), 3));

    Ok(())
}

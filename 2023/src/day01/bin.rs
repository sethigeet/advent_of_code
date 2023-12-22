use anyhow::Result;

fn part1(input: &String) -> u32 {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let mut num1 = 0;
        let mut num2 = 0;

        // Start iterating from the *start* of the line and stop when you find the 1st digit
        for ch in line.chars() {
            if let Some(d) = ch.to_digit(10) {
                num1 = d;
                break;
            }
        }

        // Start iterating from the *end* of the line and stop when you find the 1st digit
        for ch in line.chars().rev() {
            if let Some(d) = ch.to_digit(10) {
                num2 = d;
                break;
            }
        }

        sum += num1 * 10 + num2;
    }

    sum
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part2(input: &String) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut num1 = None;
        let mut num2 = 0;
        for (i, ch) in line.chars().enumerate() {
            if let Some(d) = ch.to_digit(10) {
                num1 = num1.or(Some(d));
                num2 = d;
            } else {
                for (j, dig) in DIGITS.iter().enumerate() {
                    if line[i..].starts_with(*dig) {
                        num1 = num1.or(Some((j as u32) + 1));
                        num2 = (j as u32) + 1;
                    }
                }
            }
        }
        sum += num1.unwrap() * 10 + num2;
    }

    sum
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

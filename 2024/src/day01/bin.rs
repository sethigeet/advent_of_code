use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

fn part1(input: &String) -> u32 {
    let lists: Vec<(u32, u32)> = input
        .lines()
        .map(|x| x.split_once("   ").unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect();
    let list1: Vec<u32> = lists.clone().into_iter().map(|(x, _)| x).sorted().collect();
    let list2: Vec<u32> = lists.clone().into_iter().map(|(_, y)| y).sorted().collect();

    let mut total_distance = 0;
    for (x, y) in list1.into_iter().zip(list2.into_iter()) {
        total_distance += x.abs_diff(y);
    }

    total_distance
}

fn part2(input: &String) -> u32 {
    let lists: Vec<(u32, u32)> = input
        .lines()
        .map(|x| x.split_once("   ").unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect();
    let list1: Vec<u32> = lists.clone().into_iter().map(|(x, _)| x).sorted().collect();
    let list2: Vec<u32> = lists.clone().into_iter().map(|(_, y)| y).sorted().collect();

    let mut list1_counter: HashMap<u32, u32> = HashMap::new();
    let mut list2_counter: HashMap<u32, u32> = HashMap::new();

    for (x, y) in list1.into_iter().zip(list2.into_iter()) {
        let count1 = list1_counter.entry(x).or_insert(0);
        *count1 += 1;

        let count2 = list2_counter.entry(y).or_insert(0);
        *count2 += 1;
    }

    let mut similarity_score = 0;
    for (num, val) in list1_counter.into_iter() {
        if let Some(y) = list2_counter.get(&num) {
            similarity_score += val * num * y;
        }
    }

    similarity_score
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

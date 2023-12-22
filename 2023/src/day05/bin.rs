use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct SubMap {
    src_start: u64,
    dst_start: u64,
    length: u64,
}

impl FromStr for SubMap {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u64> = input
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self {
            dst_start: nums[0],
            src_start: nums[1],
            length: nums[2],
        })
    }
}

impl SubMap {
    fn get_dst_for_src(&self, src: u64) -> Option<u64> {
        // Ensure that src in within the map range
        if !(src >= self.src_start && src < self.src_start + self.length) {
            return None;
        }

        let dst = self.dst_start + (src - self.src_start);

        Some(dst)
    }
}

#[derive(Debug)]
struct Map {
    #[allow(unused)]
    src: String,
    #[allow(unused)]
    dst: String,
    sub_maps: Vec<SubMap>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();

        let info = lines.next().unwrap();
        let names: Vec<&str> = info.split_once(" ").unwrap().0.split("-").collect();

        Ok(Self {
            dst: names[2].to_owned(),
            src: names[0].to_owned(),
            sub_maps: lines.map(|l| l.parse().unwrap()).collect(),
        })
    }
}

impl Map {
    fn get_dst_for_src(&self, src: u64) -> u64 {
        for sub_map in &self.sub_maps {
            if let Some(dst) = sub_map.get_dst_for_src(src) {
                return dst;
            }
        }

        src
    }
}

fn find_loc_for_seed(seed: u64, maps: &Vec<Map>) -> u64 {
    let mut dst = seed;
    for map in maps {
        dst = map.get_dst_for_src(dst);
    }

    dst
}

fn part1(seeds: &Vec<u64>, maps: &Vec<Map>) -> u64 {
    seeds
        .iter()
        .map(|seed| find_loc_for_seed(*seed, maps))
        .min()
        .unwrap()
}

fn part2(seeds: &Vec<u64>, maps: &Vec<Map>) -> u64 {
    let mut actual_seeds = vec![];
    for i in (0..seeds.len()).step_by(2) {
        actual_seeds.push(seeds[i]..seeds[i] + seeds[i + 1]);
    }

    actual_seeds
        .iter()
        .map(|seed_range| {
            seed_range
                .to_owned()
                .map(|seed| find_loc_for_seed(seed, maps))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() -> Result<()> {
    let input = include_str!("./sample_input.txt").to_string();
    // let input = include_str!("./input.txt").to_string();

    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds: Vec<u64> = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();
    let maps: Vec<Map> = maps.split("\n\n").map(|m| m.parse().unwrap()).collect();

    println!("Part 1: {}", part1(&seeds, &maps));
    println!("Part 2: {}", part2(&seeds, &maps));

    Ok(())
}

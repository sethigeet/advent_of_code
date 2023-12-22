use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

fn hash(seq: &str) -> u32 {
    seq.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn part1(input: &str) -> u32 {
    let sequences = input.split(",");

    sequences.map(hash).sum()
}

fn part2(input: &str) -> u32 {
    let mut boxes: HashMap<u32, Vec<Option<(&str, u32)>>> = HashMap::with_capacity(256);
    let sequences = input.split(",");

    for seq in sequences {
        if seq.contains("=") {
            let (pos, focal_len) = seq.split_once("=").unwrap();
            let b = boxes.entry(hash(pos)).or_insert(vec![]);

            let found = b.iter().find_position(|elem| match elem {
                Some((p, _)) if *p == pos => true,
                _ => false,
            });
            if let Some((id, _)) = found {
                b[id] = Some((pos, focal_len.parse().unwrap()));
            } else {
                b.push(Some((pos, focal_len.parse().unwrap())));
            }
        } else {
            let pos = &seq[..seq.len() - 1];
            let b = boxes.entry(hash(pos)).or_insert(vec![]);
            let found = b.iter().find_position(|elem| match elem {
                Some((p, _)) if *p == pos => true,
                _ => false,
            });
            if let Some((id, _)) = found {
                b[id] = None;
            }
        }
    }

    boxes
        .iter()
        .map(|(id, b)| {
            (id + 1)
                * b.iter()
                    .flatten()
                    .enumerate()
                    .map(|(id, (_, focal_len))| (id as u32 + 1) * (*focal_len))
                    .sum::<u32>()
        })
        .sum()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

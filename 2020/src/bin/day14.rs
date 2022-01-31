use std::{collections::HashMap, str::FromStr};

use anyhow::{Error, Result};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Mask {
    mask: String,
}

impl FromStr for Mask {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, val) = s.split_once(" = ").unwrap();
        if name != "mask" {
            return Err(anyhow::format_err!(
                "the given string does not specify a mask!"
            ));
        }

        Ok(Mask {
            mask: val.to_string(),
        })
    }
}

#[derive(Debug)]
struct MemVal {
    addr: u64,
    masked_addr: Vec<u64>,
    val: u64,
    masked_val: u64,
}

impl MemVal {
    fn apply_mask(&mut self, mask: &Mask) {
        let mut str_val = format!("{:0>36}", format!("{:b}", self.val));
        for (i, bit) in mask.mask.chars().enumerate() {
            if bit == 'X' {
                continue;
            }

            str_val.replace_range(i..i + 1, &bit.to_string())
        }

        self.masked_val = u64::from_str_radix(&str_val, 2).expect("str_val is not a base-2 number");
    }

    fn apply_mask_2(&mut self, mask: &Mask) {
        let mut str_val = format!("{:0>36}", format!("{:b}", self.addr));
        for (i, bit) in mask.mask.chars().enumerate() {
            if bit == '0' {
                continue;
            }

            str_val.replace_range(i..i + 1, &bit.to_string())
        }

        let mut temp: Vec<String> = vec![str_val];
        while !temp.is_empty() {
            if let Some(mut val) = temp.pop() {
                if let Some(idx) = val.find('X') {
                    val.replace_range(idx..idx + 1, "1");
                    temp.push(val.clone());
                    val.replace_range(idx..idx + 1, "0");
                    temp.push(val);
                } else {
                    self.masked_addr
                        .push(u64::from_str_radix(&val, 2).expect("val is not a base-2 number"));
                }
            }
        }
    }
}

impl FromStr for MemVal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, val) = s.split_once(" = ").unwrap();
        if name.chars().take(4).join("") != "mem[" {
            return Err(anyhow::format_err!(
                "the given string does not specify a mem val!"
            ));
        }

        let addr: u64 = name
            .replace("mem[", "")
            .replace("]", "")
            .parse()
            .expect("addr is not a u64");

        Ok(MemVal {
            addr,
            masked_addr: vec![],
            val: val.parse().expect("val is not a number"),
            masked_val: 0,
        })
    }
}

fn main() -> Result<()> {
    let lines: Vec<String> = std::fs::read_to_string("./data/inputs/14.txt")?
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut mem: HashMap<u64, u64> = HashMap::new();

    let mut mask: Mask = Mask {
        mask: "".to_string(),
    };
    for line in lines.iter() {
        if line.contains("mask") {
            mask = line.parse().expect("line is not a mask");
            continue;
        }

        let mut val: MemVal = line.parse().expect("line is not a mem val");
        val.apply_mask(&mask);
        mem.insert(val.addr, val.masked_val);
    }

    println!(
        "Part 1 -> {}",
        mem.iter().fold(0, |acc, (_, val)| acc + val)
    );

    mem = HashMap::new();

    let mut mask: Mask = Mask {
        mask: "".to_string(),
    };
    for line in lines.iter() {
        if line.contains("mask") {
            mask = line.parse().expect("line is not a mask");
            continue;
        }

        let mut val: MemVal = line.parse().expect("line is not a mem val");
        val.apply_mask_2(&mask);
        for addr in val.masked_addr.iter() {
            mem.insert(*addr, val.val);
        }
    }

    println!(
        "Part 2 -> {}",
        mem.iter().fold(0, |acc, (_, val)| acc + val)
    );

    Ok(())
}

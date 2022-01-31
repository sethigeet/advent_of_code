use std::str::FromStr;

use anyhow::{Error, Result};

fn not_in_range<T>(val: Option<T>, min: T, max: T) -> bool
where
    T: PartialOrd,
{
    val < Some(min) || val > Some(max)
}

struct Passport {
    // Birth Year
    byr: Option<u16>,
    // Issue Year
    iyr: Option<u16>,
    // Expiration Year
    eyr: Option<u16>,
    // Height
    hgt: Option<String>,
    // Hair Color
    hcl: Option<String>,
    // Eye Color
    ecl: Option<String>,
    // Passport ID (note: This is a string as it may contain leading zeroes which we need to preserve)
    pid: Option<String>,
    // Country ID
    cid: Option<u16>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn is_valid_1(&self) -> bool {
        !(self.byr == None
            || self.iyr == None
            || self.eyr == None
            || self.hgt == None
            || self.hcl == None
            || self.ecl == None
            || self.pid == None)
    }

    fn is_valid_2(&self) -> bool {
        if !self.is_valid_1() {
            return false;
        }

        if not_in_range(self.byr, 1920, 2002)
            || not_in_range(self.iyr, 2010, 2020)
            || not_in_range(self.eyr, 2020, 2030)
            || self.pid.as_ref().unwrap().len() != 9
        {
            return false;
        }

        let hgt = self.hgt.as_ref().unwrap();
        let unit = &hgt[hgt.len() - 2..];
        let hgt: u8 = hgt[..hgt.len() - 2].parse().unwrap_or(0);
        if match unit {
            "cm" => !(150..=193).contains(&hgt),
            "in" => !(59..=76).contains(&hgt),

            _ => true,
        } {
            return false;
        };

        let hcl = self.hcl.as_ref().unwrap();
        if !(hcl.len() == 7 || hcl.starts_with('#')) {
            return false;
        }
        let hcl = &hcl[1..];
        let accepted = "0123456789abcdef";
        for c in hcl.chars() {
            if !accepted.contains(c) {
                return false;
            }
        }

        let accepted = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let ecl = self.ecl.as_ref().unwrap();
        if !accepted.contains(&ecl.as_str()) {
            return false;
        }

        true
    }
}

impl FromStr for Passport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut pass = Passport::new();

        let props = s
            .lines()
            .map::<Vec<&str>, fn(&str) -> Vec<&str>>(|line| line.split(' ').collect())
            .flatten();

        for prop in props {
            let (key, val): (&str, &str) = prop.split_once(":").unwrap();

            match key {
                "byr" => pass.byr = Some(val.parse::<u16>().unwrap()),
                "iyr" => pass.iyr = Some(val.parse::<u16>().unwrap()),
                "eyr" => pass.eyr = Some(val.parse::<u16>().unwrap()),
                "hgt" => pass.hgt = Some(val.parse::<String>().unwrap()),
                "hcl" => pass.hcl = Some(val.parse::<String>().unwrap()),
                "ecl" => pass.ecl = Some(val.parse::<String>().unwrap()),
                "pid" => pass.pid = Some(val.parse::<String>().unwrap()),
                "cid" => pass.cid = Some(val.parse::<u16>().unwrap()),

                _ => panic!("invalid property found!"),
            }
        }

        Ok(pass)
    }
}

fn main() -> Result<()> {
    let passports: Vec<Passport> = std::fs::read_to_string("./data/inputs/4.txt")?
        .split("\n\n")
        .map(|block| block.parse().expect("line is not a passport"))
        .collect();

    println!(
        "Part 1 -> {}",
        passports
            .iter()
            .fold::<u16, fn(u16, &Passport) -> u16>(0, |acc, pass| {
                if pass.is_valid_1() {
                    acc + 1
                } else {
                    acc
                }
            },)
    );

    println!(
        "Part 2 -> {}",
        passports
            .iter()
            .fold::<u16, fn(u16, &Passport) -> u16>(0, |acc, pass| {
                if pass.is_valid_2() {
                    acc + 1
                } else {
                    acc
                }
            },)
    );

    Ok(())
}

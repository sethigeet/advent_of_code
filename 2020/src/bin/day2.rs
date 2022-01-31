use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
}

impl PasswordPolicy {
    fn new(min: usize, max: usize, letter: char) -> PasswordPolicy {
        PasswordPolicy { min, max, letter }
    }
}

impl FromStr for PasswordPolicy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((range, letter)) = s.split_once(" ") {
            let letter: char = letter.parse()?;

            if let Some((min, max)) = range.split_once("-") {
                let min: usize = min.parse()?;
                let max: usize = max.parse()?;

                Ok(PasswordPolicy::new(min, max, letter))
            } else {
                Err(anyhow::format_err!("could not split range"))
            }
        } else {
            Err(anyhow::format_err!("could not split policy"))
        }
    }
}

#[derive(Debug)]
struct Password {
    val: String,
    policy: PasswordPolicy,
}

impl Password {
    fn new(val: String, policy: PasswordPolicy) -> Password {
        Password { val, policy }
    }

    fn is_valid_1(&self) -> bool {
        let count = self.val.matches(&String::from(self.policy.letter)).count();

        if count < self.policy.min || count > self.policy.max {
            return false;
        }

        true
    }

    fn is_valid_2(&self) -> bool {
        let pr_at_pos_1 = self.val.chars().nth(self.policy.min - 1).unwrap() == self.policy.letter;
        let pr_at_pos_2 = self.val.chars().nth(self.policy.max - 1).unwrap() == self.policy.letter;

        if pr_at_pos_1 || pr_at_pos_2 {
            if pr_at_pos_1 && pr_at_pos_2 {
                return false;
            }

            return true;
        }

        false
    }
}

impl FromStr for Password {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((policy, val)) = s.split_once(": ") {
            let policy: PasswordPolicy = policy.parse()?;
            let val: String = String::from(val);

            Ok(Password::new(val, policy))
        } else {
            Err(anyhow::format_err!("could not split line"))
        }
    }
}

fn main() -> Result<()> {
    let passwords: Vec<Password> = std::fs::read_to_string("./data/inputs/2.txt")?
        .lines()
        .map(|line| line.parse().expect("line is not a password"))
        .collect();

    println!(
        "Part 1 -> {}",
        passwords
            .iter()
            .fold::<u16, fn(u16, &Password) -> u16>(0, |acc, pass| if pass.is_valid_1() {
                acc + 1
            } else {
                acc
            },)
    );

    println!(
        "Part 2 -> {}",
        passwords
            .iter()
            .fold::<u16, fn(u16, &Password) -> u16>(0, |acc, pass| if pass.is_valid_2() {
                acc + 1
            } else {
                acc
            },)
    );

    Ok(())
}

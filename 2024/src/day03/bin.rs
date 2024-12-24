use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Instruction {
    op: String,
    args: Option<String>,
}

impl Instruction {
    fn parse_args(&self) -> Result<(i32, i32)> {
        match &self.args {
            Some(args) => {
                let args: Vec<&str> = args.split(",").collect();
                if args.len() != 2 {
                    return Err(anyhow!("Invalid arguments"));
                }

                let a = args[0].parse::<i32>()?;
                let b = args[1].parse::<i32>()?;
                if a >= 0 && a <= 999 && b >= 0 && b <= 999 {
                    Ok((a, b))
                } else {
                    Err(anyhow!("Invalid arguments"))
                }
            }
            None => Err(anyhow!("Invalid arguments")),
        }
    }

    fn execute(&self) -> i32 {
        match &self.op[..] {
            "mul" => {
                if let Ok((a, b)) = self.parse_args() {
                    a * b
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut ans = 0;

    let mut i = 0;
    while i < input.len() - 4 {
        if &input[i..i + 4] != "mul(" {
            i += 1;
            continue;
        }
        i += 4;
        let mut j = i;
        while j < input.len() {
            if &input[j..j + 1] == ")" {
                break;
            }
            j += 1;
        }
        let instruction = Instruction {
            op: "mul".to_string(),
            args: Some(input[i..j].to_string()),
        };

        ans += instruction.execute();
    }

    ans
}

fn part2(input: &str) -> i32 {
    let mut ans = 0;

    let mut i = 0;
    let mut instruction_enabled = true;
    while i < input.len() - 4 {
        if &input[i..i + 4] == "do()" {
            i += 4;
            instruction_enabled = true;
            continue;
        }

        if i <= input.len() - 7 && &input[i..i + 7] == "don't()" {
            i += 7;
            instruction_enabled = false;
            continue;
        }

        if !instruction_enabled {
            i += 1;
            continue;
        }

        if &input[i..i + 4] != "mul(" {
            i += 1;
            continue;
        }
        i += 4;
        let mut j = i;
        while j < input.len() {
            if &input[j..j + 1] == ")" {
                break;
            }
            j += 1;
        }
        let instruction = Instruction {
            op: "mul".to_string(),
            args: Some(input[i..j].to_string()),
        };

        ans += instruction.execute();
    }

    ans
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

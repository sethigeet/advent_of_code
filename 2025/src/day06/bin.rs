use anyhow::Result;

fn part1(input: &String) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let nums = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let operators = lines
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();

    let mut answers = vec![];
    for i in 0..operators.len() {
        answers.push(match operators[i] {
            "+" => 0,
            "*" => 1,
            _ => panic!("Unknown operator"),
        });
    }
    for i in 0..nums.len() {
        for j in 0..operators.len() {
            match operators[j] {
                "+" => answers[j] += nums[i][j],
                "*" => answers[j] *= nums[i][j],
                _ => panic!("Unknown operator"),
            }
        }
    }

    answers.into_iter().sum()
}

fn part2(input: &String) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let nums = lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let operators = lines
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();
    let mut lengths = vec![0; nums[0].len()];
    for i in 0..nums.len() {
        for j in 0..nums[0].len() {
            lengths[j] = lengths[j].max(nums[i][j].len());
        }
    }

    let mut actual_nums = vec![];
    let mut offset = 0;
    for i in 0..nums[0].len() {
        let mut col = vec![0; lengths[i]];
        for j in 0..nums.len() {
            for k in 0..lengths[i] {
                let c = lines[j].chars().nth(offset + k).unwrap_or(' ');
                if c == ' ' {
                    continue;
                }

                col[k] *= 10;
                col[k] += c.to_digit(10).unwrap() as u64;
            }
        }
        actual_nums.push(col);
        offset += lengths[i] + 1;
    }

    let mut ans = 0;
    for i in 0..operators.len() {
        ans += match operators[i] {
            "+" => actual_nums[i].iter().sum::<u64>(),
            "*" => actual_nums[i].iter().product::<u64>(),
            _ => panic!("Unknown operator"),
        };
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

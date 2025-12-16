use anyhow::Result;

struct Range {
    start: u64,
    end: u64,
}

fn part1(ranges: &Vec<Range>) -> u64 {
    let mut ans = 0;
    for range in ranges {
        for i in range.start..=range.end {
            let str_i = i.to_string();
            if str_i.len() % 2 != 0 {
                continue;
            }
            let mid = str_i.len() / 2;
            let (left, right) = str_i.split_at(mid);
            if left == right {
                ans += i;
            }
        }
    }
    ans
}

fn part2(ranges: &Vec<Range>) -> u64 {
    let mut ans = 0;
    for range in ranges {
        for num in range.start..=range.end {
            let str_i = num.to_string();
            for part_len in 1..=str_i.len() / 2 {
                if str_i.len() % part_len != 0 {
                    continue;
                }

                let mut is_invalid = true;
                for part_idx in 0..(str_i.len() / part_len - 1) {
                    if str_i[part_idx * part_len..(part_idx + 1) * part_len]
                        != str_i[(part_idx + 1) * part_len..(part_idx + 2) * part_len]
                    {
                        is_invalid = false;
                        break;
                    }
                }
                if is_invalid {
                    ans += num;
                    break;
                }
            }
        }
    }
    ans
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let ranges = input
        .split(',')
        .map(|x| {
            let (start, end) = x.split_once('-').unwrap();
            let (start, end) = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());
            Range { start, end }
        })
        .collect::<Vec<Range>>();

    println!("Part 1: {}", part1(&ranges));
    println!("Part 2: {}", part2(&ranges));

    Ok(())
}

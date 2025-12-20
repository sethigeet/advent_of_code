use anyhow::Result;

#[derive(Debug, Clone)]
struct Range {
    min: u64,
    max: u64,
}

fn part1(ranges: &Vec<Range>, ids: &Vec<u64>) -> u64 {
    let mut ans = 0;
    for id in ids {
        let mut valid = false;
        for range in ranges {
            if *id >= range.min && *id <= range.max {
                valid = true;
                break;
            }
        }
        if valid {
            ans += 1;
        }
    }

    ans
}

fn part2(ranges: &Vec<Range>) -> u64 {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|r| r.min);

    let mut collapsed_ranges: Vec<Range> = vec![sorted_ranges[0].clone()];
    let mut current_end = collapsed_ranges[0].max;
    for range in sorted_ranges {
        if range.min > current_end + 1 {
            collapsed_ranges.push(Range {
                min: range.min,
                max: range.max,
            });
            current_end = range.max;
        } else if range.max > current_end {
            current_end = range.max;
            collapsed_ranges.last_mut().unwrap().max = current_end;
        }
    }

    let mut ans = 0;
    for range in collapsed_ranges {
        ans += range.max - range.min + 1;
    }

    ans
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges: Vec<Range> = ranges
        .lines()
        .map(|line| {
            let (min, max) = line.split_once("-").unwrap();
            Range {
                min: min.parse().unwrap(),
                max: max.parse().unwrap(),
            }
        })
        .collect();
    let ids: Vec<u64> = ids.lines().map(|line| line.parse().unwrap()).collect();

    println!("Part 1: {}", part1(&ranges, &ids));
    println!("Part 2: {}", part2(&ranges));

    Ok(())
}

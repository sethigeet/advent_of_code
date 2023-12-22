/*
NOTE: This problem has many different sample inputs and all of them don't work for both parts
*/

use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}
impl From<char> for Direction {
    fn from(input: char) -> Self {
        use Direction::*;

        match input {
            'L' => Left,
            'R' => Right,
            _ => unreachable!(),
        }
    }
}

type Loc<'a> = &'a str;
type Map<'a> = HashMap<Loc<'a>, (Loc<'a>, Loc<'a>)>;

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

const START_NODE: Loc = "AAA";
const END_NODE: Loc = "ZZZ";
fn part1(directions: &Vec<Direction>, map: &Map) -> u32 {
    let mut num_steps = 0;
    let mut curr_node = START_NODE;
    loop {
        let next_step = directions[num_steps % directions.len()];
        num_steps += 1;

        curr_node = match next_step {
            Direction::Left => map.get(&curr_node).unwrap().0,
            Direction::Right => map.get(&curr_node).unwrap().1,
        };

        if curr_node == END_NODE {
            break;
        }
    }

    num_steps as u32
}

fn part2(directions: &Vec<Direction>, map: &Map) -> usize {
    let curr_nodes: Vec<&Loc> = map.keys().filter(|node| node.ends_with("A")).collect();
    let mut nums = Vec::with_capacity(curr_nodes.len());

    for curr_node in curr_nodes.iter() {
        let mut num_steps = 0;
        let mut curr_node = *curr_node;
        loop {
            let next_step = directions[num_steps % directions.len()];
            num_steps += 1;

            curr_node = match next_step {
                Direction::Left => &map.get(curr_node).unwrap().0,
                Direction::Right => &map.get(curr_node).unwrap().1,
            };

            if curr_node.ends_with("Z") {
                nums.push(num_steps);
                break;
            }
        }
    }

    // Find the LCM of the numbers
    nums.into_iter()
        .reduce(|acc, num| acc * num / gcd(acc, num))
        .unwrap()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt");
    let input = include_str!("./input.txt");

    let (directions, map) = input.split_once("\n\n").unwrap();

    let directions: Vec<Direction> = directions.chars().map(|c| c.into()).collect();
    let map: Map = map.lines().fold(HashMap::new(), |mut acc, line| {
        let (loc, connections) = line.split_once(" = ").unwrap();
        acc.insert(
            loc,
            connections[1..connections.len() - 1]
                .split_once(", ")
                .unwrap(),
        );
        acc
    });

    println!("Part 1: {}", part1(&directions, &map));
    println!("Part 2: {}", part2(&directions, &map));

    Ok(())
}

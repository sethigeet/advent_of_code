use std::collections::{HashMap, HashSet};

use anyhow::Result;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Node {
    Start,
    Rock,
    Plot,
}

impl From<char> for Node {
    fn from(input: char) -> Self {
        match input {
            'S' => Node::Start,
            '#' => Node::Rock,
            '.' => Node::Plot,
            _ => unreachable!(),
        }
    }
}

type Map = HashMap<(isize, isize), Node>;

fn part1(map: &Map, dimensions: &(isize, isize), start_pos: &(isize, isize)) -> usize {
    let mut plots = HashSet::from([*start_pos]);
    for _ in 0..64 {
        plots = plots
            .iter()
            .map(|&(x, y)| {
                [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                    .into_iter()
                    .filter_map(|(x, y)| {
                        if !((0..dimensions.0).contains(&x) && (0..dimensions.1).contains(&y)) {
                            return None;
                        }

                        let node = map.get(&(x, y)).unwrap();
                        match node {
                            Node::Start | Node::Plot => Some((x, y)),
                            Node::Rock => return None,
                        }
                    })
            })
            .flatten()
            .collect();
    }

    plots.len()
}

// fn part2(map: &Map, dimensions: &(isize, isize), start_pos: &(isize, isize)) -> u32 {
//     let mut num_visits = vec![vec![0; dimensions.0 as usize]; dimensions.1 as usize];
//     num_visits[start_pos.1 as usize][start_pos.0 as usize] = 1;
//     for _ in 0..6 {
//         let temp: Vec<((isize, isize), u32)> = num_visits
//             .iter()
//             .enumerate()
//             .map(|(y, line)| {
//                 line.iter()
//                     .enumerate()
//                     .filter_map(|(x, visits)| {
//                         if visits != &0 {
//                             Some(((x as isize, y as isize), *visits))
//                         } else {
//                             None
//                         }
//                     })
//                     .collect::<Vec<((isize, isize), u32)>>()
//             })
//             .flatten()
//             .collect();
//         num_visits = vec![vec![0; dimensions.0 as usize]; dimensions.1 as usize];

//         for ((x, y), visits) in temp {
//             for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
//                 .into_iter()
//                 .filter_map(|(x, y)| {
//                     let mut x = x % dimensions.0;
//                     let mut y = y % dimensions.1;
//                     if x < 0 {
//                         x += dimensions.0;
//                     }
//                     if y < 0 {
//                         y += dimensions.1;
//                     }

//                     let node = map.get(&(x, y)).unwrap();
//                     match node {
//                         Node::Start | Node::Plot => Some((x, y)),
//                         Node::Rock => return None,
//                     }
//                 })
//             {
//                 num_visits[ny as usize][nx as usize] += visits;
//             }
//         }
//     }

//     for line in num_visits.iter() {
//         for v in line {
//             if v > &0 {
//                 print!("{}", *v);
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
//     num_visits.iter().map(|line| line.iter().sum::<u32>()).sum()
// }

fn part2(map: &Map, dimensions: &(isize, isize), start_pos: &(isize, isize)) -> usize {
    let mut plots = HashSet::from([*start_pos]);
    for _ in 0..10 {
        plots = plots
            .iter()
            .map(|&(x, y)| {
                [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                    .into_iter()
                    .filter_map(|(x, y)| {
                        let node = map
                            .get(&(x.rem_euclid(dimensions.0), y.rem_euclid(dimensions.1)))
                            .unwrap();
                        match node {
                            Node::Start | Node::Plot => Some((x, y)),
                            Node::Rock => return None,
                        }
                    })
            })
            .flatten()
            .collect();
    }

    plots.len()
}

fn main() -> Result<()> {
    let input = include_str!("./sample_input.txt").to_string();
    // let input = include_str!("./input.txt").to_string();

    let mut map: Map = HashMap::new();
    let dimensions = (
        input.lines().next().unwrap().len() as isize,
        input.lines().count() as isize,
    );
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as isize, y as isize), c.into());
        }
    }

    let start_pos = map
        .iter()
        .find(|(_, node)| node == &&Node::Start)
        .unwrap()
        .0;

    // println!("Part 1: {}", part1(&map, &dimensions, start_pos));
    println!("Part 2: {}", part2(&map, &dimensions, start_pos));

    Ok(())
}

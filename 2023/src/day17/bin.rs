use std::collections::{HashSet, VecDeque};

use anyhow::Result;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node((isize, isize), u32);

type CityMap = Vec<Vec<Node>>;

fn part1(city_map: &CityMap) -> u32 {
    let dimensions = (city_map[0].len() as isize, city_map.len() as isize);

    let res: (Vec<(Node, VecDeque<Node>)>, u32) = dijkstra(
        &(city_map[0][0], VecDeque::new()),
        |(node, prev_nodes)| {
            // Store node
            let mut prev_nodes = prev_nodes.clone();
            prev_nodes.push_front(*node);
            // Remove unecessary nodes
            if prev_nodes.len() > 4 {
                prev_nodes.pop_back();
            }

            let prev_moves: Vec<(isize, isize)> = prev_nodes
                .iter()
                .tuple_windows()
                .map(|(node1, node2)| (node1.0 .0 - node2.0 .0, node1.0 .1 - node2.0 .1))
                .collect();
            let prev_moves: Vec<(isize, isize)> = prev_moves.into_iter().unique().collect();

            let &Node((x, y), _) = node;
            let possible_ways: Vec<((Node, VecDeque<Node>), u32)> =
                [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                    .into_iter()
                    .flat_map(|pos| {
                        if !((0..dimensions.0).contains(&pos.0)
                            && (0..dimensions.1).contains(&pos.1))
                        {
                            return None;
                        }

                        if prev_nodes.len() < 3 {
                            return Some(pos);
                        }

                        if prev_moves.len() == 1
                            && prev_moves[0]
                                == (pos.0 - prev_nodes[0].0 .0, pos.1 - prev_nodes[0].0 .1)
                        {
                            return None;
                        }

                        Some(pos)
                    })
                    .map(|pos| {
                        let node = city_map[pos.1 as usize][pos.0 as usize];
                        ((node, prev_nodes.clone()), node.1)
                    })
                    .collect();

            possible_ways
        },
        |(Node((x, y), _), _)| *x == dimensions.0 - 1 && *y == dimensions.1 - 1,
    )
    .unwrap();

    let nodes: HashSet<(isize, isize)> = res.0.iter().map(|v| v.0 .0).collect();
    for y in 0..city_map.len() {
        for x in 0..city_map[0].len() {
            if nodes.contains(&(x as isize, y as isize)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    res.1
}

// fn part2(layout: &Layout) -> u32 {
//     0
// }

fn main() -> Result<()> {
    let input = include_str!("./sample_input.txt").to_string();
    // let input = include_str!("./input.txt").to_string();

    let city_map: CityMap = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Node((x as isize, y as isize), c.to_digit(10).unwrap()))
                .collect()
        })
        .collect();

    println!("Part 1: {}", part1(&city_map));
    // println!("Part 2: {}", part2(&layout));

    Ok(())
}

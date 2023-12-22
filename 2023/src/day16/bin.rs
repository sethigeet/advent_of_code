use std::collections::HashSet;

use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    RightMirror,
    LeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::RightMirror,
            '\\' => Tile::LeftMirror,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            c => panic!("Unknown tile: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Layout = Vec<Vec<Tile>>;

const START_POS: (usize, usize) = (0, 0);

fn find_next_pos(
    layout: &Layout,
    curr_pos: (usize, usize),
    direction: Direction,
    visited: &mut HashSet<((usize, usize), Direction)>,
) {
    let mut curr_pos = curr_pos.clone();
    let mut curr_dir = direction.clone();
    let mut curr_tile = layout[curr_pos.1][curr_pos.0];

    loop {
        visited.insert((curr_pos, curr_dir));

        curr_dir = match curr_tile {
            Tile::Empty => curr_dir,
            Tile::RightMirror => match curr_dir {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            Tile::LeftMirror => match curr_dir {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
            Tile::VerticalSplitter => match curr_dir {
                Direction::North | Direction::South => curr_dir,
                Direction::East | Direction::West => {
                    if curr_pos.1 != 0 {
                        find_next_pos(
                            layout,
                            (curr_pos.0, curr_pos.1 - 1),
                            Direction::North,
                            visited,
                        );
                    }
                    if curr_pos.1 != layout.len() - 1 {
                        find_next_pos(
                            layout,
                            (curr_pos.0, curr_pos.1 + 1),
                            Direction::South,
                            visited,
                        );
                    }
                    return;
                }
            },
            Tile::HorizontalSplitter => match curr_dir {
                Direction::East | Direction::West => curr_dir,
                Direction::North | Direction::South => {
                    if curr_pos.0 != layout[0].len() - 1 {
                        find_next_pos(
                            layout,
                            (curr_pos.0 + 1, curr_pos.1),
                            Direction::East,
                            visited,
                        );
                    }
                    if curr_pos.0 != 0 {
                        find_next_pos(
                            layout,
                            (curr_pos.0 - 1, curr_pos.1),
                            Direction::West,
                            visited,
                        );
                    }
                    return;
                }
            },
        };

        curr_pos = match curr_dir {
            Direction::North if curr_pos.1 != 0 => (curr_pos.0, curr_pos.1 - 1),
            Direction::South if curr_pos.1 != layout.len() - 1 => (curr_pos.0, curr_pos.1 + 1),
            Direction::East if curr_pos.0 != layout[0].len() - 1 => (curr_pos.0 + 1, curr_pos.1),
            Direction::West if curr_pos.0 != 0 => (curr_pos.0 - 1, curr_pos.1),
            _ => return,
        };
        curr_tile = layout[curr_pos.1][curr_pos.0];

        // Make sure we don't repeat any loops
        if visited.contains(&(curr_pos, curr_dir)) {
            return;
        }
    }
}

fn solve(layout: &Layout, start_pos: (usize, usize), start_dir: Direction) -> usize {
    let mut energized_tiles: HashSet<((usize, usize), Direction)> = HashSet::new();

    find_next_pos(&layout, start_pos, start_dir, &mut energized_tiles);

    let mut energized_locs = HashSet::new();
    for (loc, _) in energized_tiles {
        energized_locs.insert(loc);
    }

    energized_locs.len()
}

fn part1(layout: &Layout) -> usize {
    solve(layout, START_POS, Direction::East)
}

fn part2(layout: &Layout) -> usize {
    let mut num_energized = vec![];
    for i in 0..layout.len() {
        num_energized.push(solve(layout, (0, i), Direction::East));
        num_energized.push(solve(layout, (layout[0].len() - 1, i), Direction::West));
    }
    for i in 0..layout[0].len() {
        num_energized.push(solve(layout, (i, 0), Direction::South));
        num_energized.push(solve(layout, (i, layout.len() - 1), Direction::North));
    }

    *num_energized.iter().max().unwrap()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let layout: Layout = input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();

    println!("Part 1: {}", part1(&layout));
    println!("Part 2: {}", part2(&layout));

    Ok(())
}

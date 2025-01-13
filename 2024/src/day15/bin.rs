use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::Result;
use itertools::Itertools;

use aoc2024::grid::{Grid, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    BlockLeft,
    BlockRight,
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "#" => Ok(Tile::Wall),
            "O" => Ok(Tile::Block),
            "[" => Ok(Tile::BlockLeft),
            "]" => Ok(Tile::BlockRight),
            "." | "@" => Ok(Tile::Empty),
            _ => Err(anyhow::anyhow!("Invalid tile")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "<" => Ok(Move::Left),
            ">" => Ok(Move::Right),
            "^" => Ok(Move::Up),
            "v" => Ok(Move::Down),
            _ => Err(anyhow::anyhow!("Invalid move")),
        }
    }
}

fn part1(input: &String) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut bot_location = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .find(|(_, _, c)| *c == '@')
        .map(|(x, y, _)| Point::new(x as i32, y as i32))
        .unwrap();

    let mut map = Grid::from_str(map, |c| c.to_string().parse::<Tile>().unwrap());
    let moves = moves
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<Move>().unwrap())
                .collect::<Vec<Move>>()
        })
        .collect::<Vec<Move>>();

    'outer: for r#move in moves {
        let dir = match r#move {
            Move::Left => Point::new(-1, 0),
            Move::Right => Point::new(1, 0),
            Move::Up => Point::new(0, -1),
            Move::Down => Point::new(0, 1),
        };

        let mut new_location = bot_location + dir;
        let new_tile = map.get(&new_location.into());
        if new_tile.is_none() {
            continue;
        }
        let new_tile = new_tile.unwrap();

        match *new_tile {
            Tile::Wall => continue,
            Tile::Empty => bot_location = new_location,
            Tile::Block => {
                new_location = new_location + dir;
                while let Some(new_tile) = map.get(&new_location.into()) {
                    if *new_tile == Tile::Wall {
                        continue 'outer;
                    }
                    if *new_tile == Tile::Empty {
                        break;
                    }
                    new_location = new_location + dir;
                }

                match r#move {
                    Move::Left => {
                        for x in new_location.x..bot_location.x - 1 {
                            map.set(Point::new(x, bot_location.y).into(), Tile::Block);
                        }
                        map.set(
                            Point::new(bot_location.x - 1, bot_location.y).into(),
                            Tile::Empty,
                        );
                    }
                    Move::Right => {
                        for x in bot_location.x + 2..=new_location.x {
                            map.set(Point::new(x, bot_location.y).into(), Tile::Block);
                        }
                        map.set(
                            Point::new(bot_location.x + 1, bot_location.y).into(),
                            Tile::Empty,
                        );
                    }
                    Move::Up => {
                        for y in new_location.y..bot_location.y - 1 {
                            map.set(Point::new(bot_location.x, y).into(), Tile::Block);
                        }
                        map.set(
                            Point::new(bot_location.x, bot_location.y - 1).into(),
                            Tile::Empty,
                        );
                    }
                    Move::Down => {
                        for y in bot_location.y + 2..=new_location.y {
                            map.set(Point::new(bot_location.x, y).into(), Tile::Block);
                        }
                        map.set(
                            Point::new(bot_location.x, bot_location.y + 1).into(),
                            Tile::Empty,
                        );
                    }
                }
                bot_location = bot_location + dir;
            }
            _ => unreachable!(),
        }
    }

    map.grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(
                move |(j, cell)| {
                    if cell == &Tile::Block {
                        i * 100 + j
                    } else {
                        0
                    }
                },
            )
        })
        .sum::<usize>()
}

fn part2(input: &String) -> usize {
    let (map, moves) = input.split_once("\n\n").unwrap();

    // modify the map for part 2
    let map = &map
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '@' => vec!['@', '.'],
                    'O' => vec!['[', ']'],
                    _ => vec![c, c],
                })
                .join("")
        })
        .join("\n");

    let mut bot_location = map
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .find(|(_, _, c)| *c == '@')
        .map(|(x, y, _)| Point::new(x as i32, y as i32))
        .unwrap();

    let mut map = Grid::from_str(map.as_str(), |c| c.to_string().parse::<Tile>().unwrap());
    let moves = moves
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<Move>().unwrap())
                .collect::<Vec<Move>>()
        })
        .collect::<Vec<Move>>();

    'outer: for r#move in moves {
        let dir = match r#move {
            Move::Left => Point::new(-1, 0),
            Move::Right => Point::new(1, 0),
            Move::Up => Point::new(0, -1),
            Move::Down => Point::new(0, 1),
        };

        let mut new_location = bot_location + dir;
        let new_tile = map.get(&new_location.into());
        if new_tile.is_none() {
            continue;
        }
        let new_tile = new_tile.unwrap();

        match *new_tile {
            Tile::Wall => continue,
            Tile::Empty => bot_location = new_location,
            Tile::BlockLeft | Tile::BlockRight => match r#move {
                Move::Left | Move::Right => {
                    new_location = new_location + dir;
                    while let Some(new_tile) = map.get(&new_location.into()) {
                        if *new_tile == Tile::Wall {
                            continue 'outer;
                        }
                        if *new_tile == Tile::Empty {
                            break;
                        }
                        new_location = new_location + dir;
                    }

                    match r#move {
                        Move::Left => {
                            for x in new_location.x..bot_location.x - 1 {
                                map.set(
                                    Point::new(x, bot_location.y).into(),
                                    *map.get(&Point::new(x + 1, bot_location.y).into()).unwrap(),
                                );
                                map.set(Point::new(x + 1, bot_location.y).into(), Tile::Empty);
                            }
                        }
                        Move::Right => {
                            for x in (bot_location.x + 2..=new_location.x).rev() {
                                map.set(
                                    Point::new(x, bot_location.y).into(),
                                    *map.get(&Point::new(x - 1, bot_location.y).into()).unwrap(),
                                );
                                map.set(Point::new(x - 1, bot_location.y).into(), Tile::Empty);
                            }
                        }
                        _ => unreachable!(),
                    }
                    bot_location = bot_location + dir;
                }

                Move::Up | Move::Down => {
                    match (
                        map.get(&(new_location + Point::new(-1, 0)).into()),
                        map.get(&new_location.into()),
                        map.get(&(new_location + Point::new(1, 0)).into()),
                    ) {
                        (_, Some(Tile::Empty), _) => bot_location = new_location,

                        (Some(Tile::BlockLeft), Some(Tile::BlockRight), _)
                        | (_, Some(Tile::BlockLeft), Some(Tile::BlockRight)) => {
                            let mut to_check = match map.get(&new_location.into()) {
                                Some(Tile::BlockLeft) => {
                                    VecDeque::from([new_location + Point::new(1, 0), new_location])
                                }
                                Some(Tile::BlockRight) => {
                                    VecDeque::from([new_location + Point::new(-1, 0), new_location])
                                }
                                _ => unreachable!(),
                            };
                            let mut to_move = HashSet::new();

                            while let Some(loc) = to_check.pop_front() {
                                if let Some(new_tile) = map.get(&(loc + dir).into()) {
                                    match *new_tile {
                                        Tile::Wall => continue 'outer,
                                        Tile::Empty => {}
                                        Tile::BlockLeft => match r#move {
                                            Move::Up => {
                                                to_check.push_back(loc + Point::new(0, -1));
                                                to_check.push_back(loc + Point::new(1, -1));
                                            }
                                            Move::Down => {
                                                to_check.push_back(loc + Point::new(0, 1));
                                                to_check.push_back(loc + Point::new(1, 1));
                                            }
                                            _ => unreachable!(),
                                        },
                                        Tile::BlockRight => match r#move {
                                            Move::Up => {
                                                to_check.push_back(loc + Point::new(0, -1));
                                                to_check.push_back(loc + Point::new(-1, -1));
                                            }
                                            Move::Down => {
                                                to_check.push_back(loc + Point::new(0, 1));
                                                to_check.push_back(loc + Point::new(-1, 1));
                                            }
                                            _ => unreachable!(),
                                        },
                                        _ => unreachable!(),
                                    }
                                } else {
                                    continue 'outer;
                                }

                                to_move.insert(loc);
                            }

                            let to_move: Vec<_> = match r#move {
                                Move::Up => to_move
                                    .into_iter()
                                    .sorted_by(|a, b| {
                                        if a.y == b.y {
                                            Ord::cmp(&a.x, &b.x)
                                        } else {
                                            Ord::cmp(&a.y, &b.y)
                                        }
                                    })
                                    .collect(),
                                Move::Down => to_move
                                    .into_iter()
                                    .sorted_by(|a, b| {
                                        if a.y == b.y {
                                            Ord::cmp(&a.x, &b.x)
                                        } else {
                                            Ord::cmp(&b.y, &a.y)
                                        }
                                    })
                                    .collect(),
                                _ => unreachable!(),
                            };
                            let old_tiles: Vec<_> = to_move
                                .clone()
                                .into_iter()
                                .map(|loc| *map.get(&loc.into()).unwrap())
                                .collect();
                            for (loc, old_tile) in to_move.into_iter().zip(old_tiles.into_iter()) {
                                map.set((loc + dir).into(), old_tile);
                                map.set(loc.into(), Tile::Empty);
                            }

                            bot_location = new_location;
                        }

                        _ => continue,
                    }
                }
            },
            _ => unreachable!(),
        }
    }

    map.grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, cell)| {
                if cell == &Tile::BlockLeft {
                    i * 100 + j
                } else {
                    0
                }
            })
        })
        .sum::<usize>()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

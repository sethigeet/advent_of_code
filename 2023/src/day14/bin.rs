use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Rock {
    Rounded,
    Cube,
    Empty,
}

impl From<char> for Rock {
    fn from(input: char) -> Self {
        match input {
            'O' => Self::Rounded,
            '#' => Self::Cube,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

type Panel = Vec<Vec<Rock>>;

fn move_rocks_north(panel: &mut Panel) {
    for y in 0..panel.len() {
        for x in 0..panel[0].len() {
            match panel[y][x] {
                Rock::Rounded => {
                    for i in (0..y).rev() {
                        if panel[i][x] == Rock::Empty {
                            if i == 0 {
                                panel[y][x] = Rock::Empty;
                                panel[i][x] = Rock::Rounded;
                                break;
                            }
                            continue;
                        }

                        panel[y][x] = Rock::Empty;
                        panel[i + 1][x] = Rock::Rounded;
                        break;
                    }
                }
                Rock::Cube => continue,
                Rock::Empty => continue,
            }
        }
    }
}

fn move_rocks_south(panel: &mut Panel) {
    for y in (0..panel.len()).rev() {
        for x in 0..panel[0].len() {
            match panel[y][x] {
                Rock::Rounded => {
                    for i in y + 1..panel.len() {
                        if panel[i][x] == Rock::Empty {
                            if i == panel.len() - 1 {
                                panel[y][x] = Rock::Empty;
                                panel[i][x] = Rock::Rounded;
                                break;
                            }
                            continue;
                        }

                        panel[y][x] = Rock::Empty;
                        panel[i - 1][x] = Rock::Rounded;
                        break;
                    }
                }
                Rock::Cube => continue,
                Rock::Empty => continue,
            }
        }
    }
}

fn move_rocks_east(panel: &mut Panel) {
    for x in (0..panel[0].len()).rev() {
        for y in 0..panel.len() {
            match panel[y][x] {
                Rock::Rounded => {
                    for i in x + 1..panel[0].len() {
                        if panel[y][i] == Rock::Empty {
                            if i == panel[0].len() - 1 {
                                panel[y][x] = Rock::Empty;
                                panel[y][i] = Rock::Rounded;
                                break;
                            }
                            continue;
                        }

                        panel[y][x] = Rock::Empty;
                        panel[y][i - 1] = Rock::Rounded;
                        break;
                    }
                }
                Rock::Cube => continue,
                Rock::Empty => continue,
            }
        }
    }
}

fn move_rocks_west(panel: &mut Panel) {
    for x in 0..panel[0].len() {
        for y in 0..panel.len() {
            match panel[y][x] {
                Rock::Rounded => {
                    for i in (0..x).rev() {
                        if panel[y][i] == Rock::Empty {
                            if i == 0 {
                                panel[y][x] = Rock::Empty;
                                panel[y][i] = Rock::Rounded;
                                break;
                            }
                            continue;
                        }

                        panel[y][x] = Rock::Empty;
                        panel[y][i + 1] = Rock::Rounded;
                        break;
                    }
                }
                Rock::Cube => continue,
                Rock::Empty => continue,
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut panel: Panel = input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();

    move_rocks_north(&mut panel);

    panel.iter().enumerate().fold(0, |acc, (y, line)| {
        acc + line.iter().fold(0, |acc, rock| {
            if rock == &Rock::Rounded {
                acc + (panel.len() - y)
            } else {
                acc
            }
        })
    })
}

fn part2(input: &str) -> usize {
    let mut panel: Panel = input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();

    let mut prev_panels = HashMap::new();
    let mut i = 0;
    loop {
        move_rocks_north(&mut panel);
        move_rocks_west(&mut panel);
        move_rocks_south(&mut panel);
        move_rocks_east(&mut panel);

        if let Some(id) = prev_panels.get(&panel) {
            for _ in 0..(1_000_000_000 - (id + 1)) % (i - id) {
                move_rocks_north(&mut panel);
                move_rocks_west(&mut panel);
                move_rocks_south(&mut panel);
                move_rocks_east(&mut panel);
            }
            break;
        } else {
            prev_panels.insert(panel.clone(), i);
        }

        i += 1;
    }

    panel.iter().enumerate().fold(0, |acc, (y, line)| {
        acc + line.iter().fold(0, |acc, rock| {
            if rock == &Rock::Rounded {
                acc + (panel.len() - y)
            } else {
                acc
            }
        })
    })
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

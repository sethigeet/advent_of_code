use anyhow::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Block {
    Roll,
    Empty,
}

impl From<char> for Block {
    fn from(c: char) -> Self {
        match c {
            '@' => Block::Roll,
            '.' => Block::Empty,
            _ => panic!("Invalid character"),
        }
    }
}

fn is_roll_removable(i: usize, j: usize, grid: &Vec<Vec<Block>>) -> bool {
    let mut count = 0;
    if i > 0 && grid[i - 1][j] == Block::Roll {
        count += 1;
    }
    if i < grid.len() - 1 && grid[i + 1][j] == Block::Roll {
        count += 1;
    }
    if j > 0 && grid[i][j - 1] == Block::Roll {
        count += 1;
    }
    if j < grid[0].len() - 1 && grid[i][j + 1] == Block::Roll {
        count += 1;
    }
    if i > 0 && j > 0 && grid[i - 1][j - 1] == Block::Roll {
        count += 1;
    }
    if i > 0 && j < grid[0].len() - 1 && grid[i - 1][j + 1] == Block::Roll {
        count += 1;
    }
    if i < grid.len() - 1 && j > 0 && grid[i + 1][j - 1] == Block::Roll {
        count += 1;
    }
    if i < grid.len() - 1 && j < grid[0].len() - 1 && grid[i + 1][j + 1] == Block::Roll {
        count += 1;
    }
    count < 4
}

fn part1(grid: &Vec<Vec<Block>>) -> u32 {
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != Block::Roll {
                continue;
            }

            if is_roll_removable(i, j, &grid) {
                ans += 1;
            }
        }
    }

    ans
}

fn part2(grid: &Vec<Vec<Block>>) -> u32 {
    let mut grid = grid.clone();
    let mut ans = 0;
    loop {
        let mut any_removed = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != Block::Roll {
                    continue;
                }

                if is_roll_removable(i, j, &grid) {
                    ans += 1;
                    grid[i][j] = Block::Empty;
                    any_removed = true;
                }
            }
        }

        if !any_removed {
            break;
        }
    }

    ans
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let grid: Vec<Vec<Block>> = input
        .lines()
        .map(|line| line.chars().map(Block::from).collect())
        .collect();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));

    Ok(())
}

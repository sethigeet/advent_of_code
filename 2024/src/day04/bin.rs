use anyhow::Result;

fn part1(input: &String) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut ans = 0;

    // Horizontal
    for i in 0..grid.len() {
        for j in 0..grid[i].len() - 3 {
            if (grid[i][j] == 'X'
                && grid[i][j + 1] == 'M'
                && grid[i][j + 2] == 'A'
                && grid[i][j + 3] == 'S')
                || (grid[i][j] == 'S'
                    && grid[i][j + 1] == 'A'
                    && grid[i][j + 2] == 'M'
                    && grid[i][j + 3] == 'X')
            {
                ans += 1;
            }
        }
    }

    // Vertical
    for i in 0..grid.len() - 3 {
        for j in 0..grid[i].len() {
            if (grid[i][j] == 'X'
                && grid[i + 1][j] == 'M'
                && grid[i + 2][j] == 'A'
                && grid[i + 3][j] == 'S')
                || (grid[i][j] == 'S'
                    && grid[i + 1][j] == 'A'
                    && grid[i + 2][j] == 'M'
                    && grid[i + 3][j] == 'X')
            {
                ans += 1;
            }
        }
    }

    // Diagonal
    for i in 0..grid.len() - 3 {
        for j in 0..grid[i].len() - 3 {
            if (grid[i][j] == 'X'
                && grid[i + 1][j + 1] == 'M'
                && grid[i + 2][j + 2] == 'A'
                && grid[i + 3][j + 3] == 'S')
                || (grid[i][j] == 'S'
                    && grid[i + 1][j + 1] == 'A'
                    && grid[i + 2][j + 2] == 'M'
                    && grid[i + 3][j + 3] == 'X')
            {
                ans += 1;
            }
        }
    }

    // Anti-diagonal
    for i in 0..grid.len() - 3 {
        for j in 3..grid[i].len() {
            if (grid[i][j] == 'X'
                && grid[i + 1][j - 1] == 'M'
                && grid[i + 2][j - 2] == 'A'
                && grid[i + 3][j - 3] == 'S')
                || (grid[i][j] == 'S'
                    && grid[i + 1][j - 1] == 'A'
                    && grid[i + 2][j - 2] == 'M'
                    && grid[i + 3][j - 3] == 'X')
            {
                ans += 1;
            }
        }
    }

    ans
}

fn part2(input: &String) -> i32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut ans = 0;

    // Horizontal
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if ((grid[i - 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'M')
                && grid[i][j] == 'A'
                && (grid[i + 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'S'))
                || ((grid[i - 1][j - 1] == 'S' && grid[i - 1][j + 1] == 'S')
                    && grid[i][j] == 'A'
                    && (grid[i + 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'M'))
                || ((grid[i - 1][j - 1] == 'S' && grid[i - 1][j + 1] == 'M')
                    && grid[i][j] == 'A'
                    && (grid[i + 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M'))
                || ((grid[i - 1][j - 1] == 'M' && grid[i - 1][j + 1] == 'S')
                    && grid[i][j] == 'A'
                    && (grid[i + 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S'))
            {
                ans += 1;
            }
        }
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

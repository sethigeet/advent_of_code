use anyhow::Result;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PipeTypes {
    Minus,
    Pipe,
    F,
    J,
    Seven,
    L,
    Ground,
    Start,
}

#[derive(Debug)]
struct Pipe {
    x: usize,
    y: usize,
    typ: PipeTypes,
}

impl Pipe {
    fn from_char(typ: char, x: usize, y: usize) -> Self {
        use PipeTypes::*;
        let typ = match typ {
            '-' => Minus,
            '|' => Pipe,
            'F' => F,
            'J' => J,
            '7' => Seven,
            'L' => L,
            '.' => Ground,
            'S' => Start,
            _ => unreachable!(),
        };

        Self { x, y, typ }
    }
}

fn find_loop(
    map: &Vec<Vec<Pipe>>,
    start_pipe: &Pipe,
    initial_dir: Direction,
    dimensions: &(usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut curr_dir = initial_dir;
    let mut loop_pipes = vec![];
    let (mut x, mut y) = (start_pipe.x, start_pipe.y);

    loop {
        match curr_dir {
            Direction::North if y > 0 => y -= 1,
            Direction::South if y < dimensions.1 - 1 => y += 1,
            Direction::East if x < dimensions.0 - 1 => x += 1,
            Direction::West if x > 0 => x -= 1,
            _ => return None,
        };

        let next_pipe = &map[y][x];
        match (next_pipe.typ, curr_dir) {
            (PipeTypes::Minus, Direction::East) => curr_dir = Direction::East,
            (PipeTypes::Minus, Direction::West) => curr_dir = Direction::West,
            (PipeTypes::Pipe, Direction::North) => curr_dir = Direction::North,
            (PipeTypes::Pipe, Direction::South) => curr_dir = Direction::South,
            (PipeTypes::F, Direction::North) => curr_dir = Direction::East,
            (PipeTypes::F, Direction::West) => curr_dir = Direction::South,
            (PipeTypes::J, Direction::South) => curr_dir = Direction::West,
            (PipeTypes::J, Direction::East) => curr_dir = Direction::North,
            (PipeTypes::Seven, Direction::North) => curr_dir = Direction::West,
            (PipeTypes::Seven, Direction::East) => curr_dir = Direction::South,
            (PipeTypes::L, Direction::South) => curr_dir = Direction::East,
            (PipeTypes::L, Direction::West) => curr_dir = Direction::North,
            (PipeTypes::Ground, _) => return None,
            // NOTE: We subtract 1 here as we do not want the `PipeTypes::Start` position to be counted in the number of steps
            (PipeTypes::Start, _) => return Some(loop_pipes),
            _ => return None,
        };

        loop_pipes.push((x, y));
    }
}

fn part1(map: &Vec<Vec<Pipe>>, start_pipe: &Pipe, dimensions: &(usize, usize)) -> u32 {
    let ans = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .filter_map(|dir| find_loop(&map, start_pipe, dir, &dimensions))
    .map(|l| l.len())
    .max()
    .unwrap();

    (match ans % 2 {
        0 => ans / 2,
        1 => (ans + 1) / 2,
        _ => unreachable!(),
    }) as u32
}

fn part2(map: &Vec<Vec<Pipe>>, start_pipe: &Pipe, dimensions: &(usize, usize)) -> u32 {
    let mut loop_pipes_coord: Vec<(usize, usize)> = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .find_map(|dir| find_loop(&map, start_pipe, dir, &dimensions))
    .unwrap();
    // insert start_pipe coordinates into our vec since that is also a part of our loop
    loop_pipes_coord.push((start_pipe.x, start_pipe.y));

    let mut num_tiles = 0;
    for (y, row) in map.iter().enumerate() {
        let mut in_loop = false;
        for (x, pipe) in row.iter().enumerate() {
            if !loop_pipes_coord.contains(&(x, y)) {
                if in_loop {
                    num_tiles += 1;
                }
            } else {
                match pipe.typ {
                    PipeTypes::Minus => continue,
                    PipeTypes::Pipe => in_loop = !in_loop,
                    PipeTypes::F => in_loop = !in_loop,
                    PipeTypes::J => continue,
                    PipeTypes::Seven => in_loop = !in_loop,
                    PipeTypes::L => continue,
                    PipeTypes::Ground => continue,
                    PipeTypes::Start => continue,
                }
            }
        }
    }

    num_tiles
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let map: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Pipe::from_char(c, x, y))
                .collect()
        })
        .collect();

    // (x, y)
    let dimensions = (map[0].len(), map.len());

    let start_pipe = map
        .iter()
        .flatten()
        .find(|pipe| pipe.typ == PipeTypes::Start)
        .unwrap();

    println!("Part 1: {}", part1(&map, start_pipe, &dimensions));
    println!("Part 2: {}", part2(&map, start_pipe, &dimensions));

    Ok(())
}

use anyhow::Result;

#[derive(Debug, Clone)]
struct Node {
    is_symbol: bool,
    is_gear: bool,
    is_digit: bool,
    value: u32,
    connected_parts: Option<Vec<u32>>,
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        if let Some(d) = c.to_digit(10) {
            Node {
                is_symbol: false,
                is_gear: false,
                is_digit: true,
                value: d,
                connected_parts: None,
            }
        } else if c == '.' {
            Node {
                is_symbol: false,
                is_gear: false,
                is_digit: false,
                value: 0,
                connected_parts: None,
            }
        } else if c == '*' {
            Node {
                is_symbol: true,
                is_gear: true,
                is_digit: false,
                value: 0,
                connected_parts: Some(vec![]),
            }
        } else {
            Node {
                is_symbol: true,
                is_gear: false,
                is_digit: false,
                value: 0,
                connected_parts: None,
            }
        }
    }
}

type Engine = Vec<Vec<Node>>;

fn is_node_part_num(
    i: usize,
    j: usize,
    engine: &Engine,
    engine_dimensions: &(usize, usize),
) -> bool {
    let [y, x] = [i as isize, j as isize];
    for ny in (y - 1)..=(y + 1) {
        if ny < 0 || ny >= engine_dimensions.1 as isize {
            continue;
        }

        for nx in (x - 1)..=(x + 1) {
            if nx < 0 || nx >= engine_dimensions.0 as isize {
                continue;
            }

            let [ny, nx] = [ny as usize, nx as usize];
            if engine[ny][nx].is_symbol {
                return true;
            }
        }
    }

    false
}

fn find_gear_connected_to_node(
    i: usize,
    j: usize,
    engine: &Engine,
    engine_dimensions: &(usize, usize),
) -> Option<(usize, usize)> {
    let [y, x] = [i as isize, j as isize];
    for ny in (y - 1)..=(y + 1) {
        if ny < 0 || ny >= engine_dimensions.1 as isize {
            continue;
        }

        for nx in (x - 1)..=(x + 1) {
            if nx < 0 || nx >= engine_dimensions.0 as isize {
                continue;
            }

            let [ny, nx] = [ny as usize, nx as usize];
            if engine[ny][nx].is_symbol && engine[ny][nx].is_gear {
                return Some((ny, nx));
            }
        }
    }

    None
}

fn part1(engine: &Engine, engine_dimensions: &(usize, usize)) -> u32 {
    let mut sum = 0;
    for (i, line) in engine.iter().enumerate() {
        let mut num = 0;
        let mut is_part_num = false;
        for (j, node) in line.iter().enumerate() {
            if !node.is_digit {
                if num != 0 {
                    if is_part_num {
                        sum += num;
                        is_part_num = false;
                    }
                    num = 0;
                }

                continue;
            }

            num = num * 10 + node.value;
            if !is_part_num {
                is_part_num = is_node_part_num(i, j, &engine, &engine_dimensions);
            }

            if j == engine_dimensions.0 - 1 && is_part_num && num != 0 {
                sum += num;
            }
        }
    }

    sum
}

fn part2(engine: &Engine, engine_dimensions: &(usize, usize)) -> u32 {
    let mut sum = 0;
    let mut engine_clone = engine.clone();

    for (i, line) in engine.iter().enumerate() {
        let mut num = 0;
        let mut gear_pos = None;
        for (j, node) in line.iter().enumerate() {
            if !node.is_digit {
                if num != 0 {
                    if gear_pos.is_some() {
                        let (y, x) = gear_pos.unwrap();
                        let l: &mut Vec<Node> = &mut engine_clone[y];
                        let n: &mut Node = &mut l[x];
                        n.connected_parts.as_mut().unwrap().push(num);
                        gear_pos = None;
                    }
                    num = 0;
                }

                continue;
            }

            num = num * 10 + node.value;
            if gear_pos.is_none() {
                gear_pos = find_gear_connected_to_node(i, j, &engine, &engine_dimensions);
            }

            if j == engine_dimensions.0 - 1 && gear_pos.is_some() && num != 0 {
                let (y, x) = gear_pos.unwrap();
                engine_clone[y][x]
                    .connected_parts
                    .as_mut()
                    .unwrap()
                    .push(num);
            }
        }
    }

    for line in engine_clone.iter() {
        for node in line.iter() {
            if node.is_gear && node.connected_parts.is_some() {
                let parts = node.connected_parts.as_ref().unwrap();
                if parts.len() == 2 {
                    sum += parts.iter().product::<u32>();
                }
            }
        }
    }

    sum
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let mut engine: Engine = vec![];

    for line in input.lines() {
        let mut temp: Vec<Node> = vec![];
        for c in line.chars() {
            temp.push(c.into());
        }

        engine.push(temp);
    }

    let engine_dimensions = (engine[0].len(), engine.len());

    println!("Part 1: {}", part1(&engine, &engine_dimensions));
    println!("Part 2: {}", part2(&engine, &engine_dimensions));

    Ok(())
}

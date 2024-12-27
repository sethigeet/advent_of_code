use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    File,
    Empty,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    id: usize,
    length: usize,
    r#type: BlockType,
}

fn part1(input: &String) -> usize {
    let mut blocks: Vec<Block> = input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let r#type = if i % 2 == 0 {
                BlockType::File
            } else {
                BlockType::Empty
            };
            let length = c.to_digit(10).unwrap() as usize;
            vec![
                Block {
                    id: i / 2,
                    length,
                    r#type
                };
                length
            ]
        })
        .collect();

    let total_empty_spots = blocks
        .iter()
        .filter(|block| block.r#type == BlockType::Empty)
        .count();

    let mut num_empty_blocks_at_end = 0;
    for i in 0..blocks.len() - total_empty_spots {
        if blocks[i].r#type != BlockType::Empty {
            continue;
        }

        let mut block_to_move = blocks.len() - 1 - num_empty_blocks_at_end;
        while blocks[block_to_move].r#type == BlockType::Empty {
            num_empty_blocks_at_end += 1;
            block_to_move = blocks.len() - 1 - num_empty_blocks_at_end;
        }

        blocks.swap(i, block_to_move);
    }

    // for block in blocks.iter() {
    //     match block.r#type {
    //         BlockType::File => print!("{}", block.id),
    //         BlockType::Empty => print!("."),
    //     };
    // }
    // println!();

    blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| block.r#type != BlockType::Empty)
        .map(|(i, block)| i * block.id)
        .sum()
}

fn part2(input: &String) -> usize {
    let mut blocks: Vec<Block> = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let r#type = if i % 2 == 0 {
                BlockType::File
            } else {
                BlockType::Empty
            };
            let length = c.to_digit(10).unwrap() as usize;
            Block {
                id: i / 2,
                length,
                r#type,
            }
        })
        .collect();

    'outer: for i in 0..blocks.len() {
        if blocks[i].r#type != BlockType::Empty {
            continue;
        }

        let mut num_blocks_at_end = 0;
        let mut block_to_move = blocks.len() - 1 - num_blocks_at_end;
        while blocks[block_to_move].r#type == BlockType::Empty
            || blocks[block_to_move].length > blocks[i].length
        {
            if i == block_to_move {
                continue 'outer;
            }
            num_blocks_at_end += 1;
            block_to_move = blocks.len() - 1 - num_blocks_at_end;
        }

        blocks[i].length -= blocks[block_to_move].length;
        blocks.insert(i, blocks[block_to_move]);
        blocks[block_to_move + 1].r#type = BlockType::Empty;
    }

    // for block in blocks.iter() {
    //     match block.r#type {
    //         BlockType::File => print!("{}", format!("{}", block.id).repeat(block.length)),
    //         BlockType::Empty => print!("{}", format!(".").repeat(block.length)),
    //     };
    // }
    // println!();

    blocks
        .iter()
        .flat_map(|block| vec![block; block.length])
        .enumerate()
        .filter(|(_, block)| block.r#type != BlockType::Empty)
        .map(|(i, block)| i * block.id)
        .sum()
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

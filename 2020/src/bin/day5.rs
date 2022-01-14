use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;

const TOTAL_ROWS: usize = 128;
const TOTAL_COLS: usize = 8;

struct BoardingPass {
    row_specifier: String,
    col_specifier: String,
}

impl BoardingPass {
    fn new(row_specifier: String, col_specifier: String) -> BoardingPass {
        BoardingPass {
            row_specifier,
            col_specifier,
        }
    }

    fn get_row(self: &Self) -> usize {
        let mut range: (usize, usize) = (0, TOTAL_ROWS - 1);

        let mut i = 0;
        loop {
            let left = range.1 - range.0 + 1;
            let letter = self.row_specifier.chars().nth(i).unwrap();
            if left == 2 {
                return match letter {
                    'F' => range.0,
                    'B' => range.1,
                    _ => panic!("invalid letter specified"),
                };
            }

            let half: usize = ((range.0 + range.1 + 1) / 2) - 1;
            match letter {
                'F' => range = (range.0, half),
                'B' => range = (half + 1, range.1),
                _ => panic!("invalid letter specified"),
            }

            i += 1;
        }
    }

    fn get_col(self: &Self) -> usize {
        let mut range: (usize, usize) = (0, TOTAL_COLS - 1);

        let mut i = 0;
        loop {
            let left = range.1 - range.0 + 1;
            let letter = self.col_specifier.chars().nth(i).unwrap();
            if left == 2 {
                return match letter {
                    'L' => range.0,
                    'R' => range.1,
                    _ => panic!("invalid letter specified"),
                };
            }

            let half: usize = ((range.0 + range.1 + 1) / 2) - 1;
            match letter {
                'L' => range = (range.0, half),
                'R' => range = (half + 1, range.1),
                _ => panic!("invalid letter specified"),
            }

            i += 1;
        }
    }

    fn get_seat_id(self: &Self) -> usize {
        (self.get_row() * 8) + self.get_col()
    }
}

impl FromStr for BoardingPass {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 10 {
            Ok(BoardingPass::new(s[..7].to_string(), s[7..].to_string()))
        } else {
            Err(anyhow::format_err!(
                "len of the pass must be 10 characters!"
            ))
        }
    }
}

fn main() -> Result<()> {
    let seat_ids: Vec<usize> = std::fs::read_to_string("./data/inputs/5.txt")?
        .lines()
        .map(|line| {
            line.parse::<BoardingPass>()
                .expect("line is not a boadring pass")
        })
        .map::<usize, fn(BoardingPass) -> usize>(|p| p.get_seat_id())
        .sorted()
        .collect();

    println!("Part 1 -> {}", seat_ids.iter().max().unwrap());

    for (i, seat_id) in seat_ids.iter().enumerate() {
        if i + 1 == seat_ids.len() {
            break;
        }

        if seat_id + 1 != seat_ids[i + 1] {
            println!("Part 2 -> {}", seat_id + 1);
            break;
        }
    }

    Ok(())
}

use std::str::FromStr;

use anyhow::{Error, Result};

// NOTE: For answer to part 1 -> use TOLERANCE = 4 and get_adjacent_occupied()
//                          2 -> use TOLERANCE = 5 and get_adjacent_occupied_2()
const TOLERANCE: usize = 5;

#[derive(Clone, Debug, PartialEq)]
enum Seat {
    Empty,
    Floor,
    Occupied,
}

impl Seat {
    fn is_occupied(self: &Self) -> bool {
        if *self == Self::Occupied {
            return true;
        }

        false
    }

    fn is_floor(self: &Self) -> bool {
        if *self == Self::Floor {
            return true;
        }

        false
    }

    fn is_empty(self: &Self) -> bool {
        if *self == Self::Empty {
            return true;
        }

        false
    }

    fn occupy(self: &mut Self) {
        if self.is_occupied() || self.is_floor() {
            return;
        }

        *self = Self::Occupied
    }

    fn empty(self: &mut Self) {
        if self.is_empty() || self.is_floor() {
            return;
        }

        *self = Self::Empty
    }
}

impl FromStr for Seat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 1 {
            return Err(anyhow::format_err!("the len of the string must be 1"));
        }

        Ok(match s {
            "L" => Self::Empty,
            "." => Self::Floor,
            "#" => Self::Occupied,

            _ => panic!("invalid seat type found!"),
        })
    }
}

fn main() -> Result<()> {
    let mut rows: Vec<Vec<Seat>> = std::fs::read_to_string("./data/inputs/11.txt")?
        .lines()
        .map(|row| {
            row.chars()
                .map(|seat| {
                    seat.to_string()
                        .parse::<Seat>()
                        .expect("the char is not a seat!")
                })
                .collect()
        })
        .collect();

    let temp = &mut rows.clone();
    let mut prev: Vec<Vec<Seat>> = vec![];
    loop {
        for i in 0..rows.len() {
            for j in 0..rows[0].len() {
                let occupied_seats = get_adjacent_occupied_2(&rows, i, j)
                    .iter()
                    .filter(|seat| **seat == true)
                    .collect::<Vec<&bool>>()
                    .len();

                let curr = &mut temp[i][j];
                if curr.is_floor() {
                    continue;
                }
                if curr.is_empty() && occupied_seats == 0 {
                    curr.occupy()
                } else if curr.is_occupied() && occupied_seats >= TOLERANCE {
                    curr.empty()
                }
            }
        }

        rows = temp.clone();

        if prev == rows {
            break;
        }
        prev = temp.clone();
    }

    let unoccupied = rows
        .iter()
        .map::<Vec<bool>, fn(&Vec<Seat>) -> Vec<bool>>(|row| {
            row.iter().map(|seat| seat.is_occupied()).collect()
        })
        .flatten()
        .filter(|seat| *seat)
        .collect::<Vec<bool>>()
        .len();
    println!("Answer -> {}", unoccupied);

    Ok(())
}

// fn get_adjacent_occupied(rows: &Vec<Vec<Seat>>, row_id: usize, col_id: usize) -> Vec<bool> {
//     let mut occupied_seats: Vec<bool> = vec![];

//     // not top row
//     if row_id != 0 {
//         occupied_seats.push(rows[row_id - 1][col_id].is_occupied());

//         // not leftmost column
//         if col_id != 0 {
//             occupied_seats.push(rows[row_id - 1][col_id - 1].is_occupied())
//         }

//         // not rightmost column
//         if col_id != rows[row_id].len() - 1 {
//             occupied_seats.push(rows[row_id - 1][col_id + 1].is_occupied())
//         }
//     }

//     // not bottom row
//     if row_id != rows.len() - 1 {
//         occupied_seats.push(rows[row_id + 1][col_id].is_occupied());

//         // not leftmost column
//         if col_id != 0 {
//             occupied_seats.push(rows[row_id + 1][col_id - 1].is_occupied())
//         }

//         // not rightmost column
//         if col_id != rows[row_id].len() - 1 {
//             occupied_seats.push(rows[row_id + 1][col_id + 1].is_occupied())
//         }
//     }

//     // not leftmost column
//     if col_id != 0 {
//         occupied_seats.push(rows[row_id][col_id - 1].is_occupied())
//     }

//     // not rightmost column
//     if col_id != rows[row_id].len() - 1 {
//         occupied_seats.push(rows[row_id][col_id + 1].is_occupied())
//     }

//     occupied_seats
// }

fn get_adjacent_occupied_2(rows: &Vec<Vec<Seat>>, row_id: usize, col_id: usize) -> Vec<bool> {
    let mut occupied_seats: Vec<bool> = vec![];

    // all seats up
    for i in (0..row_id).rev() {
        let curr = &rows[i][col_id];
        if curr.is_floor() {
            continue;
        }
        if curr.is_empty() {
            occupied_seats.push(false);
            break;
        }
        if curr.is_occupied() {
            occupied_seats.push(true);
            break;
        }
    }

    // all seats down
    for i in row_id + 1..rows.len() {
        let curr = &rows[i][col_id];
        if curr.is_floor() {
            continue;
        }
        if curr.is_empty() {
            occupied_seats.push(false);
            break;
        }
        if curr.is_occupied() {
            occupied_seats.push(true);
            break;
        }
    }

    // all seats towards right
    for i in col_id + 1..rows[0].len() {
        let curr = &rows[row_id][i];
        if curr.is_floor() {
            continue;
        }
        if curr.is_empty() {
            occupied_seats.push(false);
            break;
        }
        if curr.is_occupied() {
            occupied_seats.push(true);
            break;
        }
    }

    // all seats towards left
    for i in (0..col_id).rev() {
        let curr = &rows[row_id][i];
        if curr.is_floor() {
            continue;
        }
        if curr.is_empty() {
            occupied_seats.push(false);
            break;
        }
        if curr.is_occupied() {
            occupied_seats.push(true);
            break;
        }
    }

    // all seats on left up slant
    let mut i: usize = row_id;
    let mut j: usize = col_id;
    loop {
        if i == 0 || j == 0 {
            break;
        }
        i -= 1;
        j -= 1;

        let curr = &rows[i][j];
        if curr.is_floor() {
            continue;
        }
        if curr.is_empty() {
            occupied_seats.push(false);
            break;
        }
        if curr.is_occupied() {
            occupied_seats.push(true);
            break;
        }
    }

    // all seats on right up slant
    let mut i: usize = row_id;
    let mut j: usize = col_id;
    loop {
        if i == 0 || j == (rows[0].len() - 1) {
            break;
        }
        i -= 1;
        j += 1;

        let curr = &rows[i][j];
        if curr.is_floor() {
            continue;
        }
        if curr.is_empty() {
            occupied_seats.push(false);
            break;
        }
        if curr.is_occupied() {
            occupied_seats.push(true);
            break;
        }
    }

    // all seats on right down slant
    let mut i: usize = row_id;
    let mut j: usize = col_id;
    loop {
        if i == (rows.len() - 1) || j == (rows[0].len() - 1) {
            break;
        }
        i += 1;
        j += 1;

        let curr = &rows[i][j];
        if curr.is_floor() {
            continue;
        }
        if curr.is_empty() {
            occupied_seats.push(false);
            break;
        }
        if curr.is_occupied() {
            occupied_seats.push(true);
            break;
        }
    }

    // all seats on left down slant
    let mut i: usize = row_id;
    let mut j: usize = col_id;
    loop {
        if i == (rows.len() - 1) || j == 0 {
            break;
        }
        i += 1;
        j -= 1;

        let curr = &rows[i][j];
        if curr.is_floor() {
            continue;
        }
        if curr.is_empty() {
            occupied_seats.push(false);
            break;
        }
        if curr.is_occupied() {
            occupied_seats.push(true);
            break;
        }
    }

    occupied_seats
}

use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Debug)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),

    Forward(i32),

    Left(i32),
    Right(i32),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let instruction: char = s.chars().next().unwrap();
        let val: i32 = s[1..].parse().expect("val is not a number");
        match instruction {
            'N' => Ok(Self::North(val)),
            'S' => Ok(Self::South(val)),
            'E' => Ok(Self::East(val)),
            'W' => Ok(Self::West(val)),

            'F' => Ok(Self::Forward(val)),

            'L' => Ok(Self::Left(val)),
            'R' => Ok(Self::Right(val)),

            _ => Err(anyhow::format_err!("invalid instruction provided!")),
        }
    }
}

#[derive(Debug)]
struct Pos {
    ns: i32,
    ew: i32,

    orientation: Instruction,
}

impl Pos {
    fn new() -> Self {
        Pos {
            ns: 0,
            ew: 0,
            orientation: Instruction::East(0),
        }
    }

    fn change_pos(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::North(val) => self.ns += val,
            Instruction::South(val) => self.ns -= val,
            Instruction::East(val) => self.ew += val,
            Instruction::West(val) => self.ew -= val,

            Instruction::Forward(val) => match self.orientation {
                Instruction::North(_) => self.ns += val,
                Instruction::South(_) => self.ns -= val,
                Instruction::East(_) => self.ew += val,
                Instruction::West(_) => self.ew -= val,

                _ => panic!("invalid orientation"),
            },

            Instruction::Left(val) => {
                let num_turns = (val / 90) % 4;
                match self.orientation {
                    Instruction::North(_) => {
                        if num_turns == 1 {
                            self.orientation = Instruction::West(0)
                        } else if num_turns == 2 {
                            self.orientation = Instruction::South(0)
                        } else if num_turns == 3 {
                            self.orientation = Instruction::East(0)
                        }
                    }
                    Instruction::South(_) => {
                        if num_turns == 1 {
                            self.orientation = Instruction::East(0)
                        } else if num_turns == 2 {
                            self.orientation = Instruction::North(0)
                        } else if num_turns == 3 {
                            self.orientation = Instruction::West(0)
                        }
                    }
                    Instruction::East(_) => {
                        if num_turns == 1 {
                            self.orientation = Instruction::North(0)
                        } else if num_turns == 2 {
                            self.orientation = Instruction::West(0)
                        } else if num_turns == 3 {
                            self.orientation = Instruction::South(0)
                        }
                    }
                    Instruction::West(_) => {
                        if num_turns == 1 {
                            self.orientation = Instruction::South(0)
                        } else if num_turns == 2 {
                            self.orientation = Instruction::East(0)
                        } else if num_turns == 3 {
                            self.orientation = Instruction::North(0)
                        }
                    }

                    _ => panic!("invalid orientation"),
                }
            }

            Instruction::Right(val) => {
                let num_turns = (val / 90) % 4;
                match self.orientation {
                    Instruction::North(_) => {
                        if num_turns == 1 {
                            self.orientation = Instruction::East(0)
                        } else if num_turns == 2 {
                            self.orientation = Instruction::South(0)
                        } else if num_turns == 3 {
                            self.orientation = Instruction::West(0)
                        }
                    }
                    Instruction::South(_) => {
                        if num_turns == 1 {
                            self.orientation = Instruction::West(0)
                        } else if num_turns == 2 {
                            self.orientation = Instruction::North(0)
                        } else if num_turns == 3 {
                            self.orientation = Instruction::East(0)
                        }
                    }
                    Instruction::East(_) => {
                        if num_turns == 1 {
                            self.orientation = Instruction::South(0)
                        } else if num_turns == 2 {
                            self.orientation = Instruction::West(0)
                        } else if num_turns == 3 {
                            self.orientation = Instruction::North(0)
                        }
                    }
                    Instruction::West(_) => {
                        if num_turns == 1 {
                            self.orientation = Instruction::North(0)
                        } else if num_turns == 2 {
                            self.orientation = Instruction::East(0)
                        } else if num_turns == 3 {
                            self.orientation = Instruction::South(0)
                        }
                    }

                    _ => panic!("invalid orientation"),
                }
            }
        }
    }

    fn change_pos_2(&mut self, instruction: &Instruction, waypoint_pos: &Pos) {
        match instruction {
            Instruction::Forward(val) => {
                self.ns += val * waypoint_pos.ns;
                self.ew += val * waypoint_pos.ew;
            }

            _ => panic!("invalid instruction!"),
        }
    }

    fn change_pos_2_2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Left(val) => {
                let num_turns = (val / 90) % 4;

                if num_turns == 1 {
                    let temp = self.ns;
                    self.ns = self.ew;
                    self.ew = -temp;
                } else if num_turns == 2 {
                    self.ns = -self.ns;
                    self.ew = -self.ew;
                } else if num_turns == 3 {
                    let temp = self.ns;
                    self.ns = -self.ew;
                    self.ew = temp;
                }
            }

            Instruction::Right(val) => {
                let num_turns = (val / 90) % 4;

                if num_turns == 1 {
                    let temp = self.ns;
                    self.ns = -self.ew;
                    self.ew = temp;
                } else if num_turns == 2 {
                    self.ns = -self.ns;
                    self.ew = -self.ew;
                } else if num_turns == 3 {
                    let temp = self.ns;
                    self.ns = self.ew;
                    self.ew = -temp;
                }
            }

            _ => panic!("invalid instruction!"),
        }
    }
}

fn main() -> Result<()> {
    let instructions: Vec<Instruction> = std::fs::read_to_string("./data/inputs/12.txt")?
        .lines()
        .map(|line| line.parse().expect("line is not an instruction"))
        .collect();

    let mut pos = Pos::new();
    instructions
        .iter()
        .for_each(|instruction| pos.change_pos(instruction));
    println!("Part 1 -> {}", pos.ns.abs() + pos.ew.abs());

    let mut ship_pos = Pos::new();
    let mut waypoint_pos = Pos {
        ns: 1,
        ew: 10,
        orientation: Instruction::East(0),
    };
    instructions
        .iter()
        .for_each(|instruction| match *instruction {
            Instruction::Forward(_) => ship_pos.change_pos_2(instruction, &waypoint_pos),

            Instruction::Left(_) => waypoint_pos.change_pos_2_2(instruction),
            Instruction::Right(_) => waypoint_pos.change_pos_2_2(instruction),

            _ => waypoint_pos.change_pos(instruction),
        });
    println!("Part 2 -> {}", ship_pos.ns.abs() + ship_pos.ew.abs());

    Ok(())
}

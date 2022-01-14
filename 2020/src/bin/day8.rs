use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
struct Command {
    instruction: Instruction,
}

impl Command {
    fn new(instruction: &str, arg: i32) -> Command {
        let inst = match instruction {
            "acc" => Instruction::Acc(arg),
            "jmp" => Instruction::Jmp(arg),
            "nop" => Instruction::Nop(0),

            _ => panic!("invaid instruction provided"),
        };

        Command { instruction: inst }
    }

    fn is_jmp(self: &Self) -> bool {
        if let Instruction::Jmp(_) = self.instruction {
            true
        } else {
            false
        }
    }

    fn is_nop(self: &Self) -> bool {
        if let Instruction::Nop(_) = self.instruction {
            true
        } else {
            false
        }
    }

    fn change_jmp_to_nop(self: &mut Self) {
        if !self.is_jmp() {
            panic!("command does not have a jmp instruction")
        }

        if let Instruction::Jmp(val) = self.instruction {
            self.instruction = Instruction::Nop(val)
        }
    }

    fn change_nop_to_jmp(self: &mut Self) {
        if !self.is_nop() {
            panic!("command does not have a nop instruction")
        }

        if let Instruction::Nop(val) = self.instruction {
            self.instruction = Instruction::Jmp(val)
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((instruction, arg)) = s.split_once(" ") {
            let arg: i32 = arg.parse().expect("arg in not a number");
            Ok(Command::new(instruction, arg))
        } else {
            Err(anyhow::format_err!(
                "an error occurred while trying to split the string"
            ))
        }
    }
}

fn get_acc_if_infi(cmds: &Vec<Command>, brk: bool) -> Option<i32> {
    let mut run: Vec<usize> = vec![];
    let mut acc: i32 = 0;
    let mut i: i32 = 0;
    while i < cmds.len().try_into().unwrap() {
        let idx: usize = i.try_into().expect("unable to parse i to usize");
        let cmd = &cmds[idx];
        if run.contains(&idx) {
            if brk {
                break;
            } else {
                return None;
            }
        }

        i += match cmd.instruction {
            Instruction::Acc(val) => {
                acc += val;
                1
            }
            Instruction::Jmp(val) => val,
            Instruction::Nop(_) => 1,
        };

        run.push(idx)
    }

    Some(acc)
}

fn main() -> Result<()> {
    let mut cmds: Vec<Command> = std::fs::read_to_string("./data/inputs/8.txt")?
        .lines()
        .map(|line| line.parse().expect("line is not a command"))
        .collect();

    println!("Part 1 -> {}", get_acc_if_infi(&cmds, true).unwrap());

    // NOTE: I got the answer here directly! If we would not get the answer
    // here, we would have to do this same for nop -> jmp
    let jmp_ids: Vec<usize> = cmds
        .iter()
        .enumerate()
        .filter(|(_, cmd)| cmd.is_jmp())
        .map(|(i, _)| i)
        .collect();
    let brute = jmp_ids
        .iter()
        .map(|id| {
            // change the val for now
            cmds[*id].change_jmp_to_nop();
            let ans = get_acc_if_infi(&cmds, false);
            // revert it back
            cmds[*id].change_nop_to_jmp();
            ans
        })
        .filter(|val| val != &None)
        .next()
        .expect("unable to brute-force")
        .unwrap();

    println!("Part 2 -> {}", brute);

    Ok(())
}

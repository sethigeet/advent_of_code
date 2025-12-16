use anyhow::Result;

enum Action {
    Left,
    Right,
}

fn part1(actions: &Vec<(Action, i32)>) -> i32 {
    let mut position = 50;
    let mut ans = 0;
    for (action, steps) in actions {
        match action {
            Action::Left => position -= steps,
            Action::Right => position += steps,
        }
        if position % 100 == 0 {
            ans += 1;
        }
    }

    ans
}

fn part2(actions: &Vec<(Action, i32)>) -> i32 {
    let mut position = 50;
    let mut ans = 0;
    for (action, steps) in actions {
        match action {
            Action::Left => {
                for _ in 0..*steps {
                    position -= 1;
                    if position % 100 == 0 {
                        ans += 1;
                    }
                }
            }
            Action::Right => {
                for _ in 0..*steps {
                    position += 1;
                    if position % 100 == 0 {
                        ans += 1;
                    }
                }
            }
        }
    }

    ans
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let actions = input
        .lines()
        .map(|x| {
            let action_char = x.chars().nth(0).unwrap();
            let action = match action_char {
                'L' => Action::Left,
                'R' => Action::Right,
                _ => unreachable!("Invalid action: {}", action_char),
            };
            let steps = x[1..].parse::<i32>().expect("Invalid steps");
            (action, steps)
        })
        .collect::<Vec<(Action, i32)>>();

    println!("Part 1: {}", part1(&actions));
    println!("Part 2: {}", part2(&actions));

    Ok(())
}

use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;

#[derive(Debug)]
struct Rule {
    name: String,
    allowed_vals: Vec<u32>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((name, ranges)) = s.split_once(": ") {
            let mut temp: Vec<Vec<u32>> = vec![];
            for range in ranges.split(" or ") {
                let (min, max) = range.split_once("-").expect("unable to split range");
                let (min, max) = (
                    min.parse().expect("max is not a number"),
                    max.parse().expect("max is not a number"),
                );
                temp.push(Vec::from_iter(min..=max));
            }

            Ok(Rule {
                name: name.to_string(),
                allowed_vals: temp.into_iter().flatten().collect(),
            })
        } else {
            Err(anyhow::format_err!("the rule could not be split"))
        }
    }
}

#[derive(Debug)]
struct Ticket {
    vals: Vec<u32>,
}

impl FromStr for Ticket {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Ticket {
            vals: s
                .split(',')
                .map(|val| val.parse().expect("val is not a numner"))
                .collect(),
        })
    }
}

// fn is_valid_rule(vals: &Vec<u32>, rule: &Rule) -> bool {
//     for val in vals.iter() {
//         if !rule.allowed_vals.contains(val) {
//             return false;
//         }
//     }

//     true
// }

fn check_invalid_tickets(
    my_ticket: &Ticket,
    other_tickets: &[&Ticket],
    rules: &[Rule],
    order: &[usize],
) -> bool {
    for i in 0..order.len() {
        let rule = &rules[order[i]];
        if !rule.allowed_vals.contains(&my_ticket.vals[i]) {
            return false;
        }

        for ticket in other_tickets.iter() {
            if !rule.allowed_vals.contains(&ticket.vals[i]) {
                return false;
            }
        }
    }

    true
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/inputs/16.txt")?;

    let (rules, my_ticket, other_tickets) = input.split("\n\n").collect_tuple().unwrap();
    let rules: Vec<Rule> = rules
        .lines()
        .map(|ticket| ticket.parse().expect("line is not a rule!"))
        .collect();
    let my_ticket: Ticket = my_ticket
        .lines()
        .nth(1)
        .unwrap()
        .parse()
        .expect("line is not a rule!");
    let other_tickets: Vec<Ticket> = other_tickets
        .lines()
        .skip(1)
        .map(|ticket| ticket.parse().expect("line is not a ticket!"))
        .collect();

    let allowed_vals: Vec<&u32> = rules
        .iter()
        .map(|rule| &rule.allowed_vals)
        .flatten()
        .collect();
    let mut invalid_vals: Vec<&u32> = vec![];
    my_ticket.vals.iter().for_each(|val| {
        if allowed_vals.contains(&val) {
        } else {
            invalid_vals.push(val)
        }
    });
    other_tickets.iter().for_each(|ticket| {
        ticket.vals.iter().for_each(|val| {
            if allowed_vals.contains(&val) {
            } else {
                invalid_vals.push(val)
            }
        })
    });

    println!(
        "Part 1 -> {}",
        invalid_vals.iter().fold(0, |acc, val| acc + (**val))
    );

    let other_tickets: Vec<&Ticket> = other_tickets
        .iter()
        .filter(|ticket| {
            for val in ticket.vals.iter() {
                if !allowed_vals.contains(&val) {
                    return false;
                }
            }

            true
        })
        .collect();

    for perm in (0..rules.len()).permutations(rules.len() - 1) {
        if check_invalid_tickets(&my_ticket, &other_tickets, &rules, &perm) {
            let mut ans = 1;
            for i in perm.iter() {
                if rules[*i].name.contains("depart") {
                    ans *= my_ticket.vals[*i];
                }
            }
            println!("Part 2 -> {}", ans);
            break;
        }
    }

    Ok(())
}

/*
let mut ordered_rules: HashMap<usize, Vec<&Rule>> = HashMap::new();
for i in 0..my_ticket.vals.len() {
    let mut vals: Vec<u32> = other_tickets.iter().map(|ticket| ticket.vals[i]).collect();
    vals.push(my_ticket.vals[i]);
    for rule in rules.iter() {
        if is_valid_rule(&vals, rule) {
            if let Some(valid_rules) = ordered_rules.get_mut(&i) {
                valid_rules.push(rule);
            } else {
                ordered_rules.insert(i, vec![rule]);
            }
        }
    }
}

let mut cp = ordered_rules.clone();
'iloop: for (i, valid_rules) in ordered_rules.iter() {
    if valid_rules.len() == 1 {
        cp.insert(*i, vec![valid_rules[0]]);
        continue 'iloop;
    }

    let names: Vec<&String> = ordered_rules
        .iter()
        .filter(|(j, _)| i != *j)
        .map(|(_, other_rules)| other_rules)
        .flatten()
        .map(|r| &r.name)
        .collect();

    for rule in valid_rules.iter() {
        if !names.contains(&&rule.name) {
            cp.insert(*i, vec![rule]);
            continue 'iloop;
        }
    }
}

dbg!(&cp.iter().map(|(_, rs)| rs.len()).collect::<Vec<usize>>());
*/

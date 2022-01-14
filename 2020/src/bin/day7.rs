use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug)]
struct Bag {
    name: String,
    quantity: u16,
}

impl Bag {
    fn new(name: String, quantity: u16) -> Bag {
        Bag { name, quantity }
    }
}

fn main() -> Result<()> {
    let mut bags: HashMap<String, Vec<Bag>> = HashMap::new();

    std::fs::read_to_string("./data/inputs/7.txt")?
        .lines()
        .map(|line| line.parse::<String>().expect("line is not a string"))
        .for_each(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            let name = split[0..2].join(" ");
            let contains = split[4..].join(" ");
            let contains = &contains.replace(".", "");

            if contains == "no other bags" {
                bags.insert(name, vec![]);
                return;
            }

            let contains = &contains.replace("bags", "");
            let contains = &contains.replace("bag", "");
            let contains = contains
                .split(", ")
                .map(|bag| {
                    let split = bag.split(" ").collect::<Vec<&str>>();
                    let quantity: u16 = split[0].parse().expect("unable to parse quantity");

                    Bag::new(split[1..3].join(" "), quantity)
                })
                .collect::<Vec<Bag>>();

            bags.insert(name, contains);
        });

    // Part 1
    let mut i = 0;
    for (_, contains) in bags.iter() {
        if check_if_contains(&bags, Some(contains)) {
            i += 1;
        }
    }
    println!("Part 1 -> {}", i);

    // Part 2
    println!(
        "Part 2 -> {}",
        get_num_bags_contained(&bags, Some(bags.get("shiny gold").unwrap()))
    );

    Ok(())
}

fn get_num_bags_contained(bags: &HashMap<String, Vec<Bag>>, contains: Option<&Vec<Bag>>) -> u16 {
    let contains = contains.unwrap();
    let mut total: u16 = 0;
    for bag in contains.iter() {
        let bags_inside = get_bags(bags, bag);
        total += match bags_inside {
            None => bag.quantity,
            Some(bags_inside) => {
                bag.quantity * (get_num_bags_contained(bags, Some(bags_inside)) + 1)
            }
        };
    }
    total
}

fn check_if_contains(bags: &HashMap<String, Vec<Bag>>, contains: Option<&Vec<Bag>>) -> bool {
    return match contains {
        None => false,
        Some(c) => {
            for bag in c.iter() {
                if bag.name == "shiny gold" {
                    return true;
                }

                if check_if_contains(bags, get_bags(bags, bag)) {
                    return true;
                }
            }

            false
        }
    };
}

fn get_bags<'a>(bags: &'a HashMap<String, Vec<Bag>>, bag: &Bag) -> Option<&'a Vec<Bag>> {
    for (b, contains) in bags.iter() {
        if bag.name == *b && contains.len() > 0 {
            return Some(contains);
        }
    }

    None
}

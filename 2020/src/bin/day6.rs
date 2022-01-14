use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let groups: Vec<String> = std::fs::read_to_string("./data/inputs/6.txt")?
        .split("\n\n")
        .map(|group| group.parse::<String>().expect("group is not a string"))
        .collect();

    println!(
        "Part 1 -> {}",
        groups
            .iter()
            .map(|group| {
                group
                    .split("\n")
                    .map(|person| person.parse::<String>().expect("person is not a string"))
                    .map::<Vec<char>, fn(String) -> Vec<char>>(|person| person.chars().collect())
                    .flatten()
                    .unique()
                    .collect::<Vec<char>>()
                    .len()
            })
            .sum::<usize>()
    );

    println!(
        "Part 2 -> {}",
        groups
            .iter()
            .map(|group| {
                let persons: Vec<String> = group
                    .split("\n")
                    .map(|person| person.parse::<String>().expect("person is not a string"))
                    .collect();

                let len = persons.len();
                persons
                    .iter()
                    .map::<Vec<char>, fn(&String) -> Vec<char>>(|person| person.chars().collect())
                    .flatten()
                    .counts()
                    .into_iter()
                    .filter(|val| val.1 == len)
                    .count()
            })
            .sum::<usize>()
    );

    Ok(())
}

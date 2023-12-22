use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, Clone)]
struct Card {
    winning_nums: Vec<u32>,
    my_nums: Vec<u32>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let nums = input.split_once(": ").unwrap().1;

        let (winning_nums, my_nums) = nums.split_once(" | ").unwrap();
        let winning_nums = winning_nums
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();
        let my_nums = my_nums
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();

        Ok(Card {
            winning_nums,
            my_nums,
        })
    }
}

fn part1(cards: &Vec<Card>) -> u32 {
    cards.iter().fold(0, |acc, card| {
        let num_wins = card.my_nums.iter().fold(0, |acc, num| {
            if card.winning_nums.contains(num) {
                acc + 1
            } else {
                acc
            }
        });

        if num_wins == 0 {
            acc
        } else {
            acc + 2_u32.pow(num_wins - 1)
        }
    })
}

fn part2_helper(curr_id: usize, num_wins: u32, cards: &Vec<Card>) -> u32 {
    let mut num_cards = 1;

    for id in curr_id + 1..=curr_id + (num_wins as usize) {
        let card = &cards[id];

        let num_wins = card.my_nums.iter().fold(0, |acc, num| {
            if card.winning_nums.contains(num) {
                acc + 1
            } else {
                acc
            }
        });

        if num_wins == 0 {
            num_cards += 1;
            continue;
        }

        num_cards += part2_helper(id, num_wins, cards);
    }

    num_cards
}

fn part2(cards: &Vec<Card>) -> u32 {
    cards.iter().enumerate().fold(0, |acc, (id, card)| {
        let num_wins = card.my_nums.iter().fold(0, |acc, num| {
            if card.winning_nums.contains(num) {
                acc + 1
            } else {
                acc
            }
        });

        if num_wins == 0 {
            return acc + 1; // add 1 for the original card
        }

        // NOTE: here we do not add 1 for the original card as that is being taken care of in the part2_helper func
        acc + part2_helper(id, num_wins, &cards)
    })
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt").to_string();
    let input = include_str!("./input.txt").to_string();

    let cards: Vec<Card> = input
        .lines()
        .map(|line| line.to_string().parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&cards));
    println!("Part 2: {}", part2(&cards));

    Ok(())
}

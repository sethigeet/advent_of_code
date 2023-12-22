use std::{collections::HashMap, str::FromStr};

use anyhow::Result;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(input: char) -> Self {
        use Card::*;
        match input {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
enum HandKind {
    HighCard(Vec<Card>),     // => 1 + 1 + 1 + 1 + 1
    OnePair(Vec<Card>),      // => 2 + 1 + 1 + 1
    TwoPair(Vec<Card>),      // => 2 + 2 + 1
    ThreeOfAKind(Vec<Card>), // => 3 + 1 + 1
    FullHouse(Vec<Card>),    // => 3 + 2
    FourOfAKind(Vec<Card>),  // => 4 + 1
    FiveOfAKind(Vec<Card>),  // => 5
}

impl FromStr for HandKind {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use HandKind::*;

        let cards: Vec<Card> = input.chars().map(|c| c.into()).collect();
        let num_jacks = cards
            .iter()
            .filter(|c| match c {
                Card::Jack => true,
                _ => false,
            })
            .count();
        // we must return early if all cards are jacks as logic ahead fails if the cards vector becomes empty
        if num_jacks == 5 {
            return Ok(FiveOfAKind(cards));
        }

        let mut card_counter: HashMap<Card, u32> = HashMap::new();
        for card in cards.iter() {
            match card_counter.get_mut(card) {
                Some(v) => *v += 1,
                None => {
                    card_counter.insert(*card, 1);
                }
            }
        }

        let counts: Vec<&u32> = card_counter.values().collect();
        let kind = if counts.contains(&&5) {
            FiveOfAKind(cards)
        } else if counts.contains(&&4) {
            FourOfAKind(cards)
        } else if counts.contains(&&3) && counts.contains(&&2) {
            FullHouse(cards)
        } else if counts.contains(&&3) {
            ThreeOfAKind(cards)
        } else if counts.iter().filter(|v| ***v == 2).count() == 2 {
            TwoPair(cards)
        } else if counts.contains(&&2) {
            OnePair(cards)
        } else {
            HighCard(cards)
        };

        match (kind, num_jacks) {
            (FourOfAKind(cards), 1 | 4) | (FullHouse(cards), 2 | 3) => Ok(FiveOfAKind(cards)),
            (ThreeOfAKind(cards), 1 | 3) | (TwoPair(cards), 2) => Ok(FourOfAKind(cards)),
            (TwoPair(cards), 1) => Ok(FullHouse(cards)),
            (OnePair(cards), 1 | 2) => Ok(ThreeOfAKind(cards)),
            (HighCard(cards), 1) => Ok(OnePair(cards)),
            (kind, _) => Ok(kind),
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    kind: HandKind,
    bid: u32,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (kind, bid) = input.split_once(" ").unwrap();
        Ok(Self {
            kind: kind.parse().unwrap(),
            bid: bid.parse().unwrap(),
        })
    }
}

fn part2(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input.lines().map(|line| line.parse().unwrap()).collect();

    hands.sort_by_key(|h| h.kind.clone());

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1) as u32)
}

fn main() -> Result<()> {
    // let input = include_str!("./sample_input.txt");
    let input = include_str!("./input.txt");

    println!("Part 2: {}", part2(input));

    Ok(())
}

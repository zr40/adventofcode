use std::collections::BTreeMap;

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/7_example");
const INPUT: &str = include_str!("input/7");

enum Mode {
    PartA,
    PartB,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: usize,
}

fn solve_for(input: &str, mode: Mode) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();

            let cards: Vec<_> = cards
                .chars()
                .map(|ch| match ch {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => match mode {
                        Mode::PartA => Card::J,
                        Mode::PartB => Card::Joker,
                    },
                    'T' => Card::T,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    ch => panic!("unknown card '{ch}'"),
                })
                .collect();

            let mut card_counts: BTreeMap<Card, i8> = BTreeMap::new();
            card_counts.insert(Card::A, 0);
            for card in cards.iter() {
                card_counts.insert(*card, card_counts.get(card).unwrap_or(&0) - 1);
            }
            let jokers = card_counts.remove(&Card::Joker).unwrap_or(0);

            let mut card_counts: Vec<_> = card_counts
                .into_iter()
                .map(|(card, count)| (count, card))
                .collect();
            card_counts.sort();
            card_counts[0].0 += jokers;

            let hand_type = match card_counts[0].0 {
                -5 => HandType::FiveOfAKind,
                -4 => HandType::FourOfAKind,
                -3 => match card_counts[1].0 {
                    -2 => HandType::FullHouse,
                    _ => HandType::ThreeOfAKind,
                },
                -2 => match card_counts[1].0 {
                    -2 => HandType::TwoPair,
                    _ => HandType::OnePair,
                },
                _ => HandType::HighCard,
            };

            Hand {
                hand_type,
                cards,
                bid: bid.parse().unwrap(),
            }
        })
        .collect();
    hands.sort();
    let mut rank = hands.len() + 1;
    hands
        .into_iter()
        .map(|hand| {
            rank -= 1;
            hand.bid * rank
        })
        .sum()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartA), 6440);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartA), 252052080);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartB), 5905);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartB), 252898370);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::PartA).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::PartB).into()
}

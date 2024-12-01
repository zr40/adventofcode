use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Bytes;

use crate::common::aoc::input_for;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("example/13");

#[derive(Debug, PartialEq, Eq)]
enum Item {
    List(Vec<Item>),
    Integer(u8),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Integer(left), Item::Integer(right)) => left.cmp(right),
            (Item::List(left), Item::List(right)) => {
                for (left, right) in left.iter().zip(right.iter()) {
                    match left.cmp(right) {
                        Ordering::Equal => {}
                        other => return other,
                    }
                }
                left.len().cmp(&right.len())
            }
            (Item::Integer(left), right) => Item::List(vec![Item::Integer(*left)]).cmp(right),
            (left, Item::Integer(right)) => left.cmp(&Item::List(vec![Item::Integer(*right)])),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_list(iter: &mut Peekable<Bytes<'_>>) -> Item {
    let mut items = vec![];

    assert_eq!(iter.next(), Some(b'['));

    loop {
        match iter.peek().unwrap() {
            b']' => {
                iter.next();
                return Item::List(items);
            }
            b'[' => {
                items.push(parse_list(iter));
            }
            b'1' => {
                iter.next();
                if *iter.peek().unwrap() == b'0' {
                    iter.next();
                    items.push(Item::Integer(10));
                } else {
                    items.push(Item::Integer(1));
                }
            }
            b',' => {
                iter.next();
            }
            _ => {
                items.push(Item::Integer(iter.next().unwrap() - b'0'));
            }
        }
    }
}

fn parse_packet(line: &str) -> Item {
    let mut iter = line.bytes().peekable();

    parse_list(&mut iter)
}

fn parse_input(input: &str) -> Vec<(Item, Item)> {
    let mut pairs = vec![];

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        pairs.push((parse_packet(line), parse_packet(lines.next().unwrap())));
        lines.next();
    }

    pairs
}

fn solve_a_for(input: &str) -> usize {
    let packet_pairs = parse_input(input);

    packet_pairs
        .iter()
        .enumerate()
        .map(|(index, (left, right))| match left.cmp(right) {
            Ordering::Less => index + 1,
            _ => 0,
        })
        .sum()
}

fn solve_b_for(input: &str) -> usize {
    let packet_pairs = parse_input(input);

    let mut packets = vec![];
    for (left, right) in packet_pairs {
        packets.push(left);
        packets.push(right);
    }

    let first_divider = Item::List(vec![Item::List(vec![Item::Integer(2)])]);
    let second_divider = Item::List(vec![Item::List(vec![Item::Integer(6)])]);

    packets.push(Item::List(vec![Item::List(vec![Item::Integer(2)])]));
    packets.push(Item::List(vec![Item::List(vec![Item::Integer(6)])]));
    packets.sort();

    let first_index = packets.iter().position(|p| first_divider.eq(p)).unwrap() + 1;
    let second_index = packets.iter().position(|p| second_divider.eq(p)).unwrap() + 1;

    first_index * second_index
}

#[test]
fn a_example() {
    assert_eq!(solve_a_for(EXAMPLE), 13);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_a_for(&input_for(2022, 13)), 6101);
}

fn solve_a() -> PuzzleResult {
    solve_a_for(&input_for(2022, 13)).into()
}

#[test]
fn b_example() {
    assert_eq!(solve_b_for(EXAMPLE), 140);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_b_for(&input_for(2022, 13)), 21909);
}

fn solve_b() -> PuzzleResult {
    solve_b_for(&input_for(2022, 13)).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};

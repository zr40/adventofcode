use itertools::Itertools;

use crate::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/15_example");
const INPUT: &str = include_str!("input/15");

enum Mode {
    PartA,
    PartB,
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn solve_for(input: &str, mode: Mode) -> i32 {
    let mut ingredients = vec![];
    for line in input.lines() {
        let (_, rest) = line.split_once(": capacity ").unwrap();
        let (capacity, rest) = rest.split_once(", durability ").unwrap();
        let (durability, rest) = rest.split_once(", flavor ").unwrap();
        let (flavor, rest) = rest.split_once(", texture ").unwrap();
        let (texture, calories) = rest.split_once(", calories ").unwrap();

        ingredients.push(Ingredient {
            capacity: capacity.parse().unwrap(),
            durability: durability.parse().unwrap(),
            flavor: flavor.parse().unwrap(),
            texture: texture.parse().unwrap(),
            calories: calories.parse().unwrap(),
        });
    }

    ingredients
        .iter()
        .map(|_| 1..100)
        .multi_cartesian_product()
        .filter_map(|teaspoons| {
            if teaspoons.iter().sum::<i32>() == 100 {
                let mut capacity = 0;
                let mut durability = 0;
                let mut flavor = 0;
                let mut texture = 0;
                let mut calories = 0;
                for (teaspoons, ingredient) in teaspoons.iter().zip(ingredients.iter()) {
                    capacity += teaspoons * ingredient.capacity;
                    durability += teaspoons * ingredient.durability;
                    flavor += teaspoons * ingredient.flavor;
                    texture += teaspoons * ingredient.texture;
                    calories += teaspoons * ingredient.calories;
                }
                if let Mode::PartB = mode {
                    if calories != 500 {
                        return None;
                    }
                }
                Some(capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0))
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

#[test]
fn a_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartA), 62842880);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartA), 21367368);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::PartB), 57600000);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::PartB), 1766400);
}

pub fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::PartA).into()
}

pub fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::PartB).into()
}

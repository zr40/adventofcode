use itertools::Itertools;

use crate::PuzzleResult;
use crate::day::Day;

#[cfg(test)]
const EXAMPLE: &str = include_str!("input/15_example");
const INPUT: &str = include_str!("input/15");

enum Mode {
    Unrestricted,
    Calories500,
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

    if ingredients.len() == 2 {
        solve_for_len_2(ingredients.pop().unwrap(), ingredients.pop().unwrap(), mode)
    } else if ingredients.len() == 4 {
        solve_for_len_4(
            ingredients.pop().unwrap(),
            ingredients.pop().unwrap(),
            ingredients.pop().unwrap(),
            ingredients.pop().unwrap(),
            mode,
        )
    } else {
        solve_slow(ingredients, mode)
    }
}

fn solve_for_len_2(in_a: Ingredient, in_b: Ingredient, mode: Mode) -> i32 {
    let mut best = 0;

    for a in 1..100 {
        let b = 100 - a;

        let capacity = a * in_a.capacity + b * in_b.capacity;
        let durability = a * in_a.durability + b * in_b.durability;
        let flavor = a * in_a.flavor + b * in_b.flavor;
        let texture = a * in_a.texture + b * in_b.texture;
        let calories = a * in_a.calories + b * in_b.calories;

        if let Mode::Calories500 = mode {
            if calories != 500 {
                continue;
            }
        }
        best = best.max(capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0));
    }
    best
}

fn solve_for_len_4(
    in_a: Ingredient,
    in_b: Ingredient,
    in_c: Ingredient,
    in_d: Ingredient,
    mode: Mode,
) -> i32 {
    let mut best = 0;

    for a in 1..100 {
        for b in 1..(100 - a) {
            for c in 1..(100 - a - b) {
                let d = 100 - a - b - c;

                let capacity =
                    a * in_a.capacity + b * in_b.capacity + c * in_c.capacity + d * in_d.capacity;
                let durability = a * in_a.durability
                    + b * in_b.durability
                    + c * in_c.durability
                    + d * in_d.durability;
                let flavor = a * in_a.flavor + b * in_b.flavor + c * in_c.flavor + d * in_d.flavor;
                let texture =
                    a * in_a.texture + b * in_b.texture + c * in_c.texture + d * in_d.texture;
                let calories =
                    a * in_a.calories + b * in_b.calories + c * in_c.calories + d * in_d.calories;

                if let Mode::Calories500 = mode {
                    if calories != 500 {
                        continue;
                    }
                }
                best =
                    best.max(capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0));
            }
        }
    }
    best
}

fn solve_slow(ingredients: Vec<Ingredient>, mode: Mode) -> i32 {
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
                if let Mode::Calories500 = mode {
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
    assert_eq!(solve_for(EXAMPLE, Mode::Unrestricted), 62842880);
}

#[test]
fn a_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Unrestricted), 21367368);
}

#[test]
fn b_example() {
    assert_eq!(solve_for(EXAMPLE, Mode::Calories500), 57600000);
}

#[test]
fn b_puzzle() {
    assert_eq!(solve_for(INPUT, Mode::Calories500), 1766400);
}

fn solve_a() -> PuzzleResult {
    solve_for(INPUT, Mode::Unrestricted).into()
}

fn solve_b() -> PuzzleResult {
    solve_for(INPUT, Mode::Calories500).into()
}

pub(super) static DAY: Day = Day::Separate {
    a: solve_a,
    b: solve_b,
};

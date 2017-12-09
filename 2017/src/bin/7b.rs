use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn solve(input: &str) {
    // Partial solution. Prints possible candidates having incorrect weight.

    let mut holding = HashMap::new();
    let mut weights = HashMap::new();

    for line in input.lines() {
        let mut line = line.split(" -> ");

        let program = line.next().unwrap();
        let mut parts = program.split(" (");
        let name = parts.next().unwrap();
        let weight = u32::from_str_radix(parts.next().unwrap().trim_matches(')'), 10).unwrap();

        weights.insert(name, weight);

        match line.next() {
            Some(others) => {
                let others: Vec<&str> = others.split(", ").collect();
                holding.insert(name, others);
            },
            None => {},
        }
    }

    for program in holding.keys() {
        let max = holding.get(program).unwrap().iter().max_by_key(|x| weight_of(x, &holding, &weights)).unwrap();
        let min = holding.get(program).unwrap().iter().min_by_key(|x| weight_of(x, &holding, &weights)).unwrap();

        let max_weight = weight_of(max, &holding, &weights);
        let min_weight = weight_of(min, &holding, &weights);

        if max_weight != min_weight {
            println!("----");
            for held_program in holding.get(program).unwrap() {
                println!("{} ({}): {}", held_program, weights.get(held_program).unwrap(), weight_of(held_program, &holding, &weights));
            }
        }
    }
}

fn weight_of(program: &str, holding: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, u32>) -> u32 {
    let mut weight = *weights.get(program).unwrap();
    for held_program in holding.get(program).unwrap_or(&vec!()) {
        weight += weight_of(held_program, holding, weights);
    }
    weight
}

#[test]
fn test() {
    solve("pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)");
}

fn main() {
    let mut f = File::open("input/7").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    solve(input.trim());
}

use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn solve(input: &str) -> &str {
    let mut held_by = HashMap::new();

    for line in input.split("\n") {
        let mut line = line.split(" -> ");

        let program = line.next().unwrap().split(' ').next().unwrap();

        match line.next() {
            Some(others) => {
                for other in others.split(", ") {
                    held_by.insert(other, program);
                }
            },
            None => {},
        }
    }

    let mut program = held_by.keys().next().unwrap();

    while held_by.contains_key(program) {
        program = held_by.get(program).unwrap();
    }

    program
}

#[test]
fn test() {
    assert_eq!(solve("pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)"), "tknk");
}

fn main() {
    let mut f = File::open("input/7").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}

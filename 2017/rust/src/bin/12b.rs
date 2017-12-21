use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let mut connections = HashMap::new();

    for line in input.lines() {
        let mut line = line.split(" <-> ");
        let program = u32::from_str_radix(line.next().unwrap(), 10).unwrap();
        let connected_programs: Vec<u32> = line.next().unwrap().split(", ").map(|x| u32::from_str_radix(x, 10).unwrap()).collect();

        connections.insert(program, connected_programs);
    }

    let mut visited = HashSet::new();
    let mut groups = 0;

    for program in connections.keys() {
        if visited.contains(program) {
            continue;
        }

        let mut queue = vec!(program);

        groups += 1;

        while let Some(program) = queue.pop() {
            if visited.contains(program) {
                continue;
            }

            visited.insert(program);

            for connected_program in connections.get(program).unwrap() {
                queue.push(connected_program);
            }
        }
    }

    groups
}


#[test]
fn test() {
    assert_eq!(solve("0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5"), 2);
}

fn main() {
    let mut f = File::open("input/12").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}

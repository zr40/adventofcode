use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn solve(input: &str) -> i32 {
    let mut valid = 0;

    'passphrase: for passphrase in input.lines() {
        let mut words = HashSet::new();

        for word in passphrase.split_whitespace() {
            if words.contains(word) {
                continue 'passphrase;
            }

            words.insert(word);
        }

        valid += 1;
    }

    valid
}

#[test]
fn test() {
    assert_eq!(solve("aa bb cc dd ee"), 1);
    assert_eq!(solve("aa bb cc dd aa"), 0);
    assert_eq!(solve("aa bb cc dd aaa"), 1);
}

fn main() {
    let mut f = File::open("input/4").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}

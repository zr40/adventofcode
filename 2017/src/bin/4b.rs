use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn solve(input: &str) -> i32 {
    let mut valid = 0;

    'passphrase: for passphrase in input.lines() {
        let mut words: HashSet<String> = HashSet::new();

        for word in passphrase.split_whitespace() {
            let mut chars: Vec<u8> = word.bytes().collect();
            chars.sort_unstable();

            let sorted = String::from_utf8(chars).unwrap();

            if words.contains(&sorted) {
                continue 'passphrase;
            }

            words.insert(sorted);
        }

        valid += 1;
    }

    valid
}

#[test]
fn test() {
    assert_eq!(solve("abcde fghij"), 1);
    assert_eq!(solve("abcde xyz ecdab"), 0);
    assert_eq!(solve("a ab abc abd abf abj"), 1);
    assert_eq!(solve("iiii oiii ooii oooi oooo"), 1);
    assert_eq!(solve("oiii ioii iioi iiio"), 0);
}

fn main() {
    let mut f = File::open("input/4").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}

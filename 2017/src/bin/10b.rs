use std::io::Read;
use std::fs::File;

const FINAL_LENGTHS: [u8; 5] = [17, 31, 73, 47, 23];

fn solve(input: &str) -> String {
    let mut list = Vec::with_capacity(256);

    for i in 0..256 {
        list.push(i)
    }

    let mut position: usize = 0;
    let mut skip_size: usize = 0;

    let input: Vec<usize> = input.as_bytes().iter().chain(FINAL_LENGTHS.iter()).map(|x| *x as usize).collect();

    for _ in 0..64 {
        for length in input.iter() {
            let mut front = position;
            let mut back = position + length - 1;

            while front < back {
                let temp = list[front % 256];
                list[front % 256] = list[back % 256];
                list[back % 256] = temp;

                front += 1;
                back -= 1;
            }

            position = (position + length + skip_size) % 256;
            skip_size += 1;
        }
    }

    let mut output = String::new();
    let mut list_iter = list.iter();

    for _ in 0..16 {
        let mut xor = 0;

        for _ in 0..16 {
            xor ^= list_iter.next().unwrap();
        }

        output.push_str(&format!("{:02x}", xor));
    }

    output
}

#[test]
fn test() {
    assert_eq!(solve(""), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(solve("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    assert_eq!(solve("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(solve("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

fn main() {
    let mut f = File::open("input/10").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}

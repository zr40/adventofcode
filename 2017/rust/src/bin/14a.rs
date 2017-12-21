use std::io::Read;
use std::fs::File;

const FINAL_LENGTHS: [u8; 5] = [17, 31, 73, 47, 23];

fn solve(input: &str) -> u32 {
    let mut bits_set = 0;

    for suffix in 0..128 {
        let input = format!("{}-{}", input, suffix);

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

        let mut list_iter = list.iter();

        for _ in 0..16 {
            let mut xor: usize = 0;

            for _ in 0..16 {
                xor ^= list_iter.next().unwrap();
            }

            bits_set += xor.count_ones();
        }
    }

    bits_set
}

#[test]
fn test() {
    assert_eq!(solve("flqrgnkx"), 8108);
}

fn main() {
    let mut f = File::open("input/14").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}

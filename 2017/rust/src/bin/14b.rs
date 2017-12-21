use std::io::Read;
use std::fs::File;

const FINAL_LENGTHS: [u8; 5] = [17, 31, 73, 47, 23];

fn solve(input: &str) -> u32 {
    let mut bitmap = Vec::with_capacity(128);

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

        let mut bitmap_line = Vec::with_capacity(128);

        for _ in 0..16 {
            let mut xor = 0;

            for _ in 0..16 {
                xor ^= list_iter.next().unwrap();
            }

            bitmap_line.push(xor & 128 != 0);
            bitmap_line.push(xor & 64 != 0);
            bitmap_line.push(xor & 32 != 0);
            bitmap_line.push(xor & 16 != 0);
            bitmap_line.push(xor & 8 != 0);
            bitmap_line.push(xor & 4 != 0);
            bitmap_line.push(xor & 2 != 0);
            bitmap_line.push(xor & 1 != 0);
        }

        bitmap.push(bitmap_line);
    }

    let mut regions = 0;

    for x in 0..128 {
        for y in 0..128 {
            if bitmap[x][y] {
                regions += 1;

                let mut queue = vec!((x,y));

                while let Some((x,y)) = queue.pop() {
                    if x > 0 && bitmap[x - 1][y] {
                        queue.push((x - 1, y));
                        bitmap[x - 1][y] = false;
                    }
                    if x < 127 && bitmap[x + 1][y] {
                        queue.push((x + 1, y));
                        bitmap[x + 1][y] = false;
                    }
                    if y > 0 && bitmap[x][y - 1] {
                        queue.push((x, y - 1));
                        bitmap[x][y - 1] = false;
                    }
                    if y < 127 && bitmap[x][y + 1] {
                        queue.push((x, y + 1));
                        bitmap[x][y + 1] = false;
                    }
                }
            }
        }
    }

    regions
}

#[test]
fn test() {
    assert_eq!(solve("flqrgnkx"), 1242);
}

fn main() {
    let mut f = File::open("input/14").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}

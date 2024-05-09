use super::knot_hash::knot_hash;
use crate::day::Day;
use crate::puzzle_result::PuzzleResult;

#[cfg(test)]
const EXAMPLE: &str = "flqrgnkx";
const INPUT: &str = "vbqugkhl";

fn solve_for(input: &str) -> (u32, u32) {
    let mut bits_set = 0;
    let mut bitmap = Vec::with_capacity(128);

    for suffix in 0..128 {
        let list = knot_hash(&format!("{input}-{suffix}"));
        let mut list_iter = list.iter();

        let mut bitmap_line = Vec::with_capacity(128);

        for _ in 0..16 {
            let mut xor = 0;

            for _ in 0..16 {
                xor ^= list_iter.next().unwrap();
            }

            bits_set += xor.count_ones();

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

                let mut queue = vec![(x, y)];

                while let Some((x, y)) = queue.pop() {
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

    (bits_set, regions)
}

#[test]
fn example() {
    assert_eq!(solve_for(EXAMPLE), (8108, 1242));
}

#[test]
fn puzzle() {
    assert_eq!(solve_for(INPUT), (8148, 1180));
}

fn solve_both() -> (PuzzleResult, PuzzleResult) {
    let (a, b) = solve_for(INPUT);
    (a.into(), b.into())
}

pub(super) static DAY: Day = Day::Pair(solve_both);

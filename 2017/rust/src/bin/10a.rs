use std::io::Read;
use std::fs::File;

fn solve(input: &str) -> usize {
    solve_size(256, input)
}

fn solve_size(list_size: usize, input: &str) -> usize {
    let mut list = Vec::with_capacity(list_size);

    for i in 0..list_size {
        list.push(i)
    }

    let mut position: usize = 0;
    let mut skip_size: usize = 0;

    for length in input.split(",").map(|x| usize::from_str_radix(x, 10).unwrap()) {
        let mut front = position;
        let mut back = position + length - 1;

        while front < back {
            let temp = list[front % list_size];
            list[front % list_size] = list[back % list_size];
            list[back % list_size] = temp;

            front += 1;
            back -= 1;
        }

        position = (position + length + skip_size) % list_size;
        skip_size += 1;
    }

    list[0] * list[1]
}

#[test]
fn test() {
    assert_eq!(solve_size(5, "3,4,1,5"), 12);
}

fn main() {
    let mut f = File::open("input/10").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}

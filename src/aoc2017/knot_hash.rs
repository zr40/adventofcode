const FINAL_LENGTHS: [u8; 5] = [17, 31, 73, 47, 23];

pub(super) fn knot_hash(input: &str) -> Vec<usize> {
    let input: Vec<usize> = input
        .as_bytes()
        .iter()
        .chain(FINAL_LENGTHS.iter())
        .map(|x| *x as usize)
        .collect();

    knot_rounds(input, 256, 64)
}

pub(super) fn knot_rounds(input: Vec<usize>, list_size: usize, count: usize) -> Vec<usize> {
    let mut list = Vec::with_capacity(list_size);

    for i in 0..list_size {
        list.push(i);
    }

    let mut position: usize = 0;
    let mut skip_size: usize = 0;

    for _ in 0..count {
        for length in &input {
            let mut front = position;
            let mut back = position + length - 1;

            while front < back {
                list.swap(front % list_size, back % list_size);

                front += 1;
                back -= 1;
            }

            position = (position + length + skip_size) % list_size;
            skip_size += 1;
        }
    }
    list
}

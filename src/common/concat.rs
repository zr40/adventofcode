pub fn concat_num(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10() + 1) + b
}

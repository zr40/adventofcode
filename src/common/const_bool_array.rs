pub(crate) const fn str_to_bool_array<const N: usize>(s: &str) -> [bool; N] {
    let bytes = s.as_bytes();

    let mut result = [false; N];

    let mut byte_index = 0;
    let mut result_index = 0;
    while byte_index < bytes.len() {
        match bytes[byte_index] {
            b'#' => {
                result[result_index] = true;
                byte_index += 1;
                result_index += 1;
            }
            b'.' => {
                result[result_index] = false;
                byte_index += 1;
                result_index += 1;
            }
            b'\n' | b' ' => {
                byte_index += 1;
            }
            _ => panic!(),
        }
    }

    assert!(result_index == N, "incorrect number of values");

    result
}

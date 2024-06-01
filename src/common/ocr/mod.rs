use super::const_bool_array::str_to_bool_array;

struct Character<const N: usize> {
    ch: char,
    pixels: [bool; N],
}

const CHARS: &[Character<{ 5 * 6 }>] = &[
    Character {
        ch: 'A',
        pixels: str_to_bool_array(include_str!("chars/A")),
    },
    Character {
        ch: 'E',
        pixels: str_to_bool_array(include_str!("chars/E")),
    },
    Character {
        ch: 'F',
        pixels: str_to_bool_array(include_str!("chars/F")),
    },
    Character {
        ch: 'I',
        pixels: str_to_bool_array(include_str!("chars/I")),
    },
    Character {
        ch: 'J',
        pixels: str_to_bool_array(include_str!("chars/J")),
    },
    Character {
        ch: 'K',
        pixels: str_to_bool_array(include_str!("chars/K")),
    },
    Character {
        ch: 'P',
        pixels: str_to_bool_array(include_str!("chars/P")),
    },
    Character {
        ch: 'R',
        pixels: str_to_bool_array(include_str!("chars/R")),
    },
    Character {
        ch: 'Y',
        pixels: str_to_bool_array(include_str!("chars/Y")),
    },
];

pub(crate) fn ocr(pixels: &[bool], width: usize) -> String {
    const W: usize = 5;
    const H: usize = 6;

    let mut s = String::new();

    let mut idx = 0;

    while idx < width {
        let mut found = false;

        'ch: for ch in CHARS {
            for y in 0..H {
                for x in 0..W {
                    if pixels[idx + x + y * width] != ch.pixels[x + y * W] {
                        continue 'ch;
                    }
                }
            }

            s.push(ch.ch);
            found = true;
            break;
        }

        if !found {
            let mut grid = String::new();
            for y in 0..H {
                grid.push('\n');
                for x in 0..W {
                    grid.push(if pixels[idx + x + y * width] {
                        '#'
                    } else {
                        '.'
                    });
                }
            }
            panic!("could not recognize character{grid}");
        }

        idx += W;
    }

    s
}

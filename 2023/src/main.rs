#![feature(btree_extract_if)]
#![feature(float_next_up_down)]

use std::env::args;

use crate::puzzles::PUZZLES;

mod puzzles;

fn main() {
    if args().len() == 1 {
        for (name, func) in &PUZZLES {
            print!("{name}: ");
            func();
        }
    } else {
        for arg in args() {
            if let Some(func) = PUZZLES.get(&arg) {
                print!("{arg}: ");
                func();
            }
        }
    }
}

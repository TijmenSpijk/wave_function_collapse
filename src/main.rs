use std::env;

use crate::sudoku::*;

mod cell;
mod sudoku;

fn main() {
    let mut sudoku = Sudoku::new(env::args()).unwrap();
    sudoku.display();
    println!();
    sudoku.display_entropy();
    println!();
    for _ in 0..100 {
        sudoku.solve();
        println!();
    }
    println!("{}", sudoku.check());
}

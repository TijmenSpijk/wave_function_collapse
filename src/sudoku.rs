use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Read;

use rand::{seq::*, thread_rng};

use crate::cell::*;

#[derive(Clone, Copy)]
pub struct Sudoku {
    grid: [[Cell; 9]; 9],
}

impl Sudoku {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let filepath = match args.next() {
            Some(arg) => arg,
            None => "input.txt".to_string(),
        };

        let mut content = String::new();

        let mut file = match File::open(&filepath) {
            Err(err) => panic!("couldn't open file {}: {}", filepath, err),
            Ok(file) => file,
        };

        match file.read_to_string(&mut content) {
            Err(err) => panic!("couldn't read content {}: {}", filepath, err),
            Ok(_) => (),
        };

        let mut grid: [[Cell; 9]; 9] = [[Cell::new(0, (0, 0)); 9]; 9];

        let mut i = 0;
        let mut j = 0;

        content.split('\n').for_each(|row| {
            row.split_whitespace().for_each(|num| {
                grid[i][j] = Cell::new(num.parse().unwrap(), (i, j));
                i += 1;
            });
            i = 0;
            j += 1;
        });

        let sudoku = Sudoku { grid };

        Ok(sudoku)
    }

    pub fn display(&self) {
        for j in 0..9 {
            if j % 3 == 0 && j != 0 {
                println!("- - - - - - - - - - -")
            }
            for i in 0..9 {
                if i % 3 == 0 && i != 0 {
                    print!("| ")
                }
                self.grid[i][j].display();
            }
            println!();
        }
    }

    pub fn display_entropy(&self) {
        for j in 0..9 {
            if j % 3 == 0 && j != 0 {
                println!("- - - - - - - - - - -")
            }
            for i in 0..9 {
                if i % 3 == 0 && i != 0 {
                    print!("| ")
                }
                self.grid[i][j].display_entropy();
            }
            println!();
        }
    }

    pub fn check(&self) -> bool {
        for j in 0..9 {
            for i in 0..9 {
                let mut neighbors: HashSet<u8> = HashSet::new();
                for cell in self.get_row((i, j)) {
                    if cell != self.grid[i][j] {
                        match cell.get_num() {
                            Some(num) => neighbors.insert(num),
                            None => false,
                        };
                    }
                }

                for cell in self.get_column((i, j)) {
                    if cell != self.grid[i][j] {
                        match cell.get_num() {
                            Some(num) => neighbors.insert(num),
                            None => false,
                        };
                    }
                }

                for cell in self.get_box((i, j)) {
                    if cell != self.grid[i][j] {
                        match cell.get_num() {
                            Some(num) => neighbors.insert(num),
                            None => false,
                        };
                    }
                }
                for neighbor in neighbors {
                    if neighbor == match self.grid[i][j].get_num() {
                        Some(num) => num,
                        None => 10
                    } {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Sudoku {
    fn get_row(&self, (_, y): (usize, usize)) -> Vec<Cell> {
        let mut row: Vec<Cell> = Vec::new();
        for x in 0..9 {
            row.push(self.grid[x][y]);
        }
        row
    }

    fn get_column(&self, (x, _): (usize, usize)) -> Vec<Cell> {
        let mut column: Vec<Cell> = Vec::new();
        for y in 0..9 {
            column.push(self.grid[x][y]);
        }
        column
    }

    fn get_box(&self, (x, y): (usize, usize)) -> Vec<Cell> {
        let (box_x, box_y) = (x / 3 * 3, y / 3 * 3);
        let mut sudoku_box: Vec<Cell> = Vec::new();
        for j in 0..3 {
            for i in 0..3 {
                sudoku_box.push(self.grid[i + box_x][j + box_y]);
            }
        }
        sudoku_box
    }

    fn collapse(&mut self) {
        for j in 0..9 {
            for i in 0..9 {
                let mut neighbors: HashSet<u8> = HashSet::new();
                for cell in self.get_row((i, j)) {
                    match cell.get_num() {
                        Some(num) => neighbors.insert(num),
                        None => false,
                    };
                }

                for cell in self.get_column((i, j)) {
                    match cell.get_num() {
                        Some(num) => neighbors.insert(num),
                        None => false,
                    };
                }

                for cell in self.get_box((i, j)) {
                    match cell.get_num() {
                        Some(num) => neighbors.insert(num),
                        None => false,
                    };
                }

                self.grid[i][j].collapse(neighbors);
            }
        }
    }
}

impl Sudoku {
    fn get_lowest_entropy(&self) -> Vec<(usize, usize)> {
        let mut lowest_entropy = 10;
        let mut options = Vec::new();
        for j in 0..9 {
            for i in 0..9 {
                let cell = self.grid[i][j];
                let entropy = cell.entropy();
                if !cell.get_fixed() && entropy > 0 {
                    if entropy < lowest_entropy {
                        lowest_entropy = entropy;
                        options.clear();
                        options.push((i, j));
                    } else if entropy == lowest_entropy {
                        options.push((i, j));
                    }
                }
            }
        }
        println!("{:?}", options);
        options
    }

    pub fn solve(&mut self) {
        self.collapse();
        let options = self.get_lowest_entropy();
        let (i, j) = match options.choose(&mut thread_rng()) {
            Some(option) => option,
            None => return,
        };
        self.grid[*i][*j].set_num();
        self.display();
        println!();
        self.display_entropy();
        println!();
        self.collapse();
    }
}

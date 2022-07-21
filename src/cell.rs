use rand::{seq::*, thread_rng};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pos: (usize, usize),
    num: Option<u8>,
    fixed: bool,
    options: [Option<u8>; 9],
}

impl Cell {
    pub fn new(num: u8, pos: (usize, usize)) -> Self {
        let mut options: [Option<u8>; 9] = [None; 9];
        for i in 0..9 {
            options[i as usize] = Some(i + 1);
        }

        match num {
            0 => Cell {
                pos,
                num: None,
                fixed: false,
                options,
            },
            _ => Cell {
                pos,
                num: Some(num),
                fixed: true,
                options: [None; 9],
            },
        }
    }

    pub fn set_num(&mut self) {
        let mut options = self.options.to_vec();
        options.retain(|x| *x != None);
        self.num = match options.choose(&mut thread_rng()) {
            Some(num) => {
                println!("{:?} {:?}", self.pos, num);
                *num
            }
            None => None,
        };

        for i in 0..9 {
            self.options[i] = None
        }
    }

    pub fn get_num(&self) -> Option<u8> {
        self.num
    }

    pub fn get_fixed(&self) -> bool {
        self.fixed
    }

    pub fn display(&self) {
        match self.num {
            None => print!("  "),
            Some(num) => print!("{} ", num),
        }
    }

    pub fn display_entropy(&self) {
        print!("{} ", self.entropy())
    }

    pub fn collapse(&mut self, neighbors: HashSet<u8>) {
        for cell in neighbors {
            self.options[(cell - 1) as usize] = None
        }
    }

    pub fn entropy(&self) -> usize {
        let mut entropy = 0;
        for option in self.options {
            match option {
                Some(_) => entropy += 1,
                None => (),
            }
        }
        entropy
    }
}

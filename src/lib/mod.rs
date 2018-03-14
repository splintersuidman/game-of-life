extern crate rand;

mod life_106;

use std::fs::File;
use std::io::Read;
use self::life_106::Parser;

pub struct GameOfLife {
    pub board: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl GameOfLife {
    /// Return a new GameOfLife instance.
    pub fn new(width: usize, height: usize) -> GameOfLife {
        let mut board: Vec<Vec<bool>> = Vec::new();
        for _ in 0..height {
            let mut row: Vec<bool> = Vec::new();
            for _ in 0..width {
                row.push(false);
            }
            board.push(row);
        }

        GameOfLife {
            board,
            width: width as usize,
            height: height as usize
        }
    }

    /// Convert board to a String.
    /// Allow dead code here, because this function is only used in cli.rs.
    #[allow(dead_code)]
    pub fn to_string_with_alive(&self, alive: char) -> String {
        let mut s = String::new();

        for row in &self.board {
            for x in row {
                s.push(if *x { alive } else { ' ' });
            }
            s.push('\n');
        }

        s
    }

    /// Init board with only dead cells.
    /// All alive cells will be killed.
    pub fn init_empty(mut self) -> Self {
        for y in 1..self.height - 1 {
            for x in 1..self.height - 1 {
                self.board[y][x] = false;
            }
        }
        self
    }

    /// Randomly init the board, with a specified chance.
    /// A random u8 will be picked, and if it is greater than `chance`, the current cell will be
    /// alive.
    pub fn init_randomly(mut self, chance: u8) -> Self {
        self = self.init_empty();

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if rand::random::<u8>() > chance {
                    self.board[y][x] = true;
                }
            }
        }
        self
    }

    /// Init the game of life board from a Life 1.06 file.
    pub fn init_with_file<S>(mut self, filename: S) -> Result<Self, String>
    where
        S: AsRef<str>
    {
        // Read file and get rules from them.
        let mut file = match File::open(filename.as_ref()) {
            Ok(f) => f,
            Err(e) => return Err(format!("could not open file: {}", e)),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(e) => return Err(format!("could not read file to string: {}", e)),
        };

        if !Parser::is_life_106_file(contents.clone()) {
            return Err(format!("file is not a Life 1.06: missing header `#Life 1.06`"));
        }

        let cell_rules = Parser::parse_life_106_file(contents)?;

        self = self.init_empty();

        let origin = (self.width / 2, self.height / 2);

        for (x, y) in cell_rules {
            let x = x + origin.0 as isize;
            let y = y + origin.1 as isize;

            if x > 0 && x < self.width as isize && y > 0 && y < self.height as isize {
                self.board[y as usize][x as usize] = true;
            }
        }

        Ok(self)
    }

    /// Update the board using the game of life rules.
    pub fn update(&mut self) {
        // Count neighbours for all cells.
        let mut neighbours: Vec<Vec<i32>> = Vec::new();
        for y in 1..self.height - 1 {
            let mut row: Vec<i32> = Vec::new();

            for x in 1..self.width - 1 {
                let mut number_of_neighbours = 0;
                for i in 0..3 {
                    for j in 0..3 {
                        let i: usize = (y as isize + i as isize - 1) as usize;
                        let j: usize = (x as isize + j as isize - 1) as usize;
                        if self.board[i][j] {
                            number_of_neighbours += 1;
                        }
                    }
                }

                if self.board[y][x] {
                    number_of_neighbours -= 1;
                }

                row.push(number_of_neighbours);
            }
            neighbours.push(row);
        }

        // Update cells based on their neighbour count.
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let number_of_neighbours = neighbours[y - 1][x - 1];
                if self.board[y][x] {
                    if number_of_neighbours < 2 || number_of_neighbours > 3 {
                        self.board[y][x] = false;
                    }
                } else {
                    if number_of_neighbours == 3 {
                        self.board[y][x] = true;
                    }
                }
            }
        }
    }
}
extern crate rand;
extern crate rayon;

pub mod parsers;

use self::rayon::prelude::*;

pub struct GameOfLife {
    pub board: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
    pub name: Option<String>,
}

impl GameOfLife {
    /// Return a new GameOfLife instance.
    pub fn new(width: usize, height: usize) -> GameOfLife {
        let board: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| false).collect())
            .collect();

        GameOfLife {
            board,
            width: width as usize,
            height: height as usize,
            name: None,
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
    pub fn init_empty(&mut self) -> &mut Self {
        use std::iter;

        self.board = iter::repeat(iter::repeat(false).take(self.width).collect())
            .take(self.height)
            .collect();

        self
    }

    /// Randomly init the board, with a specified chance.
    /// A random u8 will be picked, and if it is greater than `chance`, the current cell will be
    /// alive.
    pub fn init_randomly(&mut self, chance: u8) -> &mut Self {
        use std::iter;
        use self::rand::Rng;

        self.init_empty();

        let mut rng = rand::thread_rng();

        self.board = iter::repeat(
            iter::repeat(())
                .map(|_| rng.gen::<u8>() > chance)
                .take(self.width)
                .collect(),
        ).take(self.height)
            .collect();

        self
    }

    /// Init the game of life board from a file.
    pub fn init_with_file<S>(&mut self, filename: S) -> Result<&mut Self, String>
    where
        S: AsRef<str>,
    {
        let pattern = parsers::Pattern::from_file(filename)?;
        if let Some(name) = pattern.name {
            self.name = Some(name);
        }

        self.init_empty();

        let origin = (self.width / 2, self.height / 2);

        for (x, y) in pattern.cells {
            let x = (x + origin.0 as isize) as usize;
            let y = (y + origin.1 as isize) as usize;

            if x > 0 && x < self.width && y > 0 && y < self.height {
                self.board[y][x] = true;
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
        let width = self.width;
        self.board
            .par_iter_mut()
            .enumerate()
            .skip(1)
            .take(self.height - 2)
            .for_each(|(y, row)| {
                row.par_iter_mut()
                    .enumerate()
                    .skip(1)
                    .take(width - 2)
                    .for_each(|(x, cell)| {
                        let number_of_neighbours = neighbours[y - 1][x - 1];
                        if *cell {
                            if number_of_neighbours < 2 || number_of_neighbours > 3 {
                                *cell = false;
                            }
                        } else if number_of_neighbours == 3 {
                            *cell = true;
                        }
                    });
            });
    }
}

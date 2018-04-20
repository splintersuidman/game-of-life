extern crate ndarray;
extern crate rand;
extern crate rayon;

pub mod parsers;

use self::ndarray::prelude::*;
use self::rand::Rng;
use self::rayon::prelude::*;
use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

impl Into<bool> for CellState {
    fn into(self) -> bool {
        self == CellState::Alive
    }
}

impl From<bool> for CellState {
    fn from(value: bool) -> Self {
        if value {
            CellState::Alive
        } else {
            CellState::Dead
        }
    }
}

impl ::std::ops::Not for CellState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            CellState::Dead => CellState::Alive,
            CellState::Alive => CellState::Dead,
        }
    }
}

pub struct GameOfLife {
    pub board: Array2<CellState>,
    pub width: usize,
    pub height: usize,
    pub name: Option<String>,
}

impl GameOfLife {
    /// Return a new GameOfLife instance.
    pub fn new(width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            board: Array2::from_elem((width, height), CellState::Dead),
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

        for y in 0..self.height {
            for x in 0..self.width {
                s.push(if self.board[(x, y)] == CellState::Alive {
                    alive
                } else {
                    ' '
                });
            }
            s.push('\n');
        }

        s
    }

    /// Init board with only dead cells.
    /// All alive cells will be killed.
    pub fn init_empty(&mut self) -> &mut Self {
        self.board = Array2::from_elem((self.width, self.height), CellState::Dead);

        self
    }

    /// Randomly init the board, with a specified chance.
    /// A random u8 will be picked, and if it is greater than `chance`, the current cell will be
    /// alive.
    pub fn init_randomly(&mut self, chance: u8) -> &mut Self {
        let mut rng = rand::weak_rng();

        self.board
            .iter_mut()
            .for_each(|cell| *cell = (rng.gen::<u8>() > chance).into());

        // Clear borders.
        for y in 1..self.height - 1 {
            self.board[(0, y)] = CellState::Dead;
            self.board[(self.width - 1, y)] = CellState::Dead;
        }

        for x in 1..self.width - 1 {
            self.board[(x, 0)] = CellState::Dead;
            self.board[(x, self.height - 1)] = CellState::Dead;
        }

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
                self.board[(x, y)] = CellState::Alive;
            }
        }

        Ok(self)
    }

    /// Update the board using the game of life rules.
    pub fn update(&mut self) {
        // Count neighbours for all cells.
        let mut neighbours: Vec<Vec<i32>> = iter::repeat(())
            .take(self.height)
            .map(|_| iter::repeat(0).take(self.width).collect())
            .collect();

        neighbours.par_iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, cell)| {
                if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
                    return;
                }

                let mut number_of_neighbours = 0;
                for i in -1..1 + 1 {
                    for j in -1..1 + 1 {
                        let x: usize = (x as isize + j as isize) as usize;
                        let y: usize = (y as isize + i as isize) as usize;
                        if self.board[(x, y)] == CellState::Alive {
                            number_of_neighbours += 1;
                        }
                    }
                }

                if self.board[(x, y)] == CellState::Alive {
                    number_of_neighbours -= 1;
                }

                *cell = number_of_neighbours
            })
        });

        // Update cells based on their neighbour count.
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let cell = &mut self.board[(x, y)];
                let neighbours = neighbours[y][x];
                if *cell == CellState::Alive {
                    if neighbours < 2 || neighbours > 3 {
                        *cell = CellState::Dead;
                    }
                } else if neighbours == 3 {
                    *cell = CellState::Alive;
                }
            }
        }
    }
}

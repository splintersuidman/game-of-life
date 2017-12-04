extern crate rand;

pub struct GameOfLife {
    pub board: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl GameOfLife {
    // Return a new GameOfLife instance.
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

    // Convert board to a String.
    // Allow dead code here, because this function is only used in cli.rs.
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

    // Randomly init the board, with a specified chance.
    pub fn random_init(mut self, chance: u8) -> Self {
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if rand::random::<u8>() > chance {
                    self.board[y][x] = true;
                }
            }
        }
        self
    }

    // Update the board using the game of life rules.
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

extern crate terminal_size;

mod lib;

use lib::GameOfLife;
use terminal_size::*;

const ALIVE: char = '*';
const HELP_MESSAGE: &str = "game-of-life v0.1.0
by Splinter Suidman

game-of-life emulates John Conway's game of life.

Flags:
  --help | -h
    Show this screen.
  --width [w] : u16
    Change the width of the board.
    Default: terminal width.
  --height [h] : u16
    Change the height of the board.
    Default: terminal height.
  --chance [c] : u8
    Chance for randomly initialising board.
    Example: with '--chance 128' passed, cells will have a 50% chance of living.
    Default: 220.
";

fn main() {
    let mut args = std::env::args().skip(1);

    // Defaults
    let mut width: u16 = 0;
    let mut height: u16 = 0;
    let mut chance: u8 = 220;

    if let Some((Width(w), Height(h))) = terminal_size() {
        width = w - 1;
        height = h - 1;
    }

    // Command line arguments parsing.
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("{}", HELP_MESSAGE);
                std::process::exit(1);
            }
            "--width" => if let Some(w) = args.next() {
                width = w.trim().parse().unwrap();
            },
            "--height" => if let Some(h) = args.next() {
                height = h.trim().parse().unwrap();
            },
            "--chance" => if let Some(c) = args.next() {
                chance = c.trim().parse().unwrap();
            },
            _ => {
                println!("Error: unknowm flag '{}'", arg);
            }
        }
    }

    let mut game_of_life = GameOfLife::new(width as usize, height as usize).random_init(chance);

    loop {
        // Clear screen.
        println!("\x1B[{}A", height + 1);

        game_of_life.update();

        let output = game_of_life.to_string_with_alive(ALIVE);
        print!("{}", output);
    }
}

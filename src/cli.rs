extern crate terminal_size;

mod lib;

use lib::GameOfLife;
use terminal_size::*;

const ALIVE: char = '*';
const HELP_MESSAGE: &str = "game-of-life v0.3.0
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
  --sleep [s] : u64
    The amount of milliseconds the thread sleeps between every frame.
    Default: None.
  --file [f] : path
    The file that contains the board.
    If this flag is passed, the board will be initialised with the board in the given file.
    Default: None.
";

fn main() {
    let mut args = std::env::args().skip(1);

    // Defaults
    let mut width: u16 = 0;
    let mut height: u16 = 0;
    let mut chance: u8 = 220;
    let mut sleep: Option<u64> = None;
    let mut file: Option<String> = None;

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
            "--sleep" => if let Some(s) = args.next() {
                sleep = Some(s.trim().parse::<u64>().unwrap());
            },
            "--file" => if let Some(f) = args.next() {
                file = Some(f);
            },
            _ => {
                println!("Error: unknowm flag '{}'", arg);
            }
        }
    }

    let mut game_of_life = if let Some(f) = file {
        GameOfLife::new(width as usize, height as usize)
            .init_with_file(f)
            .unwrap()
    } else {
        GameOfLife::new(width as usize, height as usize).init_randomly(chance)
    };

    loop {
        // Clear screen.
        println!("\x1B[{}A", height + 1);

        game_of_life.update();

        let output = game_of_life.to_string_with_alive(ALIVE);
        print!("{}", output);

        if let Some(sleep) = sleep {
            let sleep = std::time::Duration::from_millis(sleep);
            std::thread::sleep(sleep);
        }
    }
}

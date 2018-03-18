extern crate piston_window;

mod lib;

use std::env;
use lib::GameOfLife;
use piston_window::*;

const HELP_MESSAGE: &str = "game-of-life v0.2.0
by Splinter Suidman

game-of-life emulates John Conway's game of life.

Press Escape to exit, press Space or a mouse button to reinitialise grid randomly.

Flags:
  --help | -h
    Show this screen.
  --width [w] : u32
    Change the width of the board (in cells).
    Default: 50.
  --height [h] : u32
    Change the height of the board (in cells).
    Default: 50.
  --cell-width [cw] : u32
    Change width of a cell (in pixels).
    Default: 10.
  --chance [c] : u8
    Chance for randomly initialising board.
    Example: with '--chance 128' passed, cells will have a 50% chance of living.
    Default: 220.
  --fps [f] : u64
    The amount of updates and frames that should be performed per second.
    It is possible that this number is not reached because your computer can't handle it.
    Default: 10.
  --file [f] : path
    The file that contains the board.
    If this flag is passed, the board will be initialised with the board in the given file.
    Default: None.
";

fn main() {
    let mut args = env::args().skip(1);

    // Defaults
    let mut width: u32 = 50;
    let mut height: u32 = 50;
    let mut cell_width: u32 = 10;
    let mut chance: u8 = 220;
    let mut fps: u64 = 10;
    let mut file: Option<String> = None;

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
            "--cell-width" => if let Some(cw) = args.next() {
                cell_width = cw.trim().parse().unwrap();
            },
            "--chance" => if let Some(c) = args.next() {
                chance = c.trim().parse().unwrap();
            },
            "--fps" => if let Some(f) = args.next() {
                fps = f.trim().parse().unwrap();
            },
            "--file" => if let Some(f) = args.next() {
                file = Some(f);
            },
            _ => {
                println!("Error: unknowm flag `{}`", arg);
            }
        }
    }

    let mut game_of_life = if let Some(f) = file.clone() {
        GameOfLife::new(width as usize, height as usize)
            .init_with_file(f)
            .unwrap()
    } else {
        GameOfLife::new(width as usize, height as usize).init_randomly(chance)
    };

    // Create window.
    let mut window: PistonWindow =
        WindowSettings::new("Game of Life", [width * cell_width, height * cell_width])
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Set event loop settings
    let mut settings = window.get_event_settings();
    settings.set_ups(fps);
    settings.set_max_fps(fps);
    window.set_event_settings(settings);

    // Convert cell_width to f64.
    let cell_width = f64::from(cell_width);

    // Event loop.
    while let Some(e) = window.next() {
        // Key press for resetting grid.
        if let Some(button) = e.press_args() {
            use piston_window::Button::{Keyboard, Mouse};
            use piston_window::Key;

            match button {
                Keyboard(Key::Space) | Mouse(_) => {
                    if let Some(f) = file.clone() {
                        game_of_life = game_of_life.init_with_file(f).unwrap();
                    } else {
                        game_of_life = game_of_life.init_randomly(chance);
                    }
                }
                _ => (),
            }
        }

        // Drawing.
        window.draw_2d(&e, |c, g| {
            clear([1.; 4], g);

            for y in 0..game_of_life.board.len() {
                for x in 0..game_of_life.board[y].len() {
                    if game_of_life.board[y][x] {
                        rectangle(
                            [0., 0., 0., 1.],
                            [
                                (x as f64) * cell_width,
                                (y as f64) * cell_width,
                                cell_width,
                                cell_width,
                            ],
                            c.transform,
                            g,
                        );
                    }
                }
            }
        });

        game_of_life.update();
    }
}

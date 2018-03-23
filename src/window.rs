extern crate piston_window;

mod lib;

use std::env;
use lib::GameOfLife;
use piston_window::*;

const HELP_MESSAGE: &str = "game-of-life v0.3.0
by Splinter Suidman

game-of-life emulates John Conway's game of life.

Press Escape to exit, press Space or a mouse button to reinitialise grid.

Flags:
  --help
    Show this screen.
  --width [w] | -w [w]
    : u32
    Change the width of the board (in cells).
    Default: 50.
  --height [h] | -h [h]
    : u32
    Change the height of the board (in cells).
    Default: 50.
  --cell-width [cw] | -cw [cw]
    : u32
    Change width of a cell (in pixels).
    Default: 10.
  --chance [c] | -ch [ch]
    : u8
    Chance for randomly initialising board.
    Example: with '--chance 128' passed, cells will have a 50% chance of living.
    Default: 220.
  --fps [f] | -fps [f]
    : u64
    The amount of updates and frames that should be performed per second.
    This is the maximum frames per second; that is, the actual fps could be less.
    Default: 24.
  --file [f] | -f [f]
    : path
    The file that contains the board.
    If this flag is passed, the board will be initialised with the board in the given file.
    Default: None.
  --colour [c] | --color [c] | -c [c]
    : u32
    Change the foreground colour of the cells.
    The colour should be passed as a hexidecimal RGB colour, example: FFFFFF for white, 000000 for black.
    Default: FFFFFF (white).
  --background [c] | -bg [c]
    : u32
    Change the background colour.
    The colour should be passed as a hexidecimal RGB colour, example: FFFFFF for white, 000000 for black.
    Default: 000000 (black).
";

fn main() {
    let mut args = env::args().skip(1);

    // Defaults
    let mut width: u32 = 50;
    let mut height: u32 = 50;
    let mut cell_width: u32 = 10;
    let mut chance: u8 = 220;
    let mut fps: u64 = 24;
    let mut colour: u32 = 0x000000; // Black.
    let mut background: u32 = 0xFFFFFF; // White.
    let mut file: Option<String> = None;

    // Command line arguments parsing.
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" => {
                println!("{}", HELP_MESSAGE);
                std::process::exit(1);
            }
            "--width" | "-w" => if let Some(w) = args.next() {
                width = w.trim().parse().unwrap();
            },
            "--height" | "-h" => if let Some(h) = args.next() {
                height = h.trim().parse().unwrap();
            },
            "--cell-width" | "-cw" => if let Some(cw) = args.next() {
                cell_width = cw.trim().parse().unwrap();
            },
            "--chance" | "-ch" => if let Some(c) = args.next() {
                chance = c.trim().parse().unwrap();
            },
            "--colour" | "--color" | "-c" => if let Some(c) = args.next() {
                // Parse hexadecimal.
                colour = u32::from_str_radix(&c, 16).unwrap();
            },
            "--background" | "-bg" => if let Some(c) = args.next() {
                // Parse hexadecimal.
                background = u32::from_str_radix(&c, 16).unwrap();
            },
            "--fps" | "-fps" => if let Some(f) = args.next() {
                fps = f.trim().parse().unwrap();
            },
            "--file" | "-f" => if let Some(f) = args.next() {
                file = Some(f);
            },
            _ => {
                println!("Error: unknowm flag `{}`", arg);
            }
        }
    }

    let mut game_of_life = GameOfLife::new(width as usize, height as usize);

    if let Some(f) = file.clone() {
        game_of_life.init_with_file(f).unwrap();
    } else {
        game_of_life.init_randomly(chance);
    }

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

    // Convert colour to f64's.
    let bg_red: f32 = ((background & 0xFF0000) >> 16) as f32 / 255.0;
    let bg_green: f32 = ((background & 0x00FF00) >> 8) as f32 / 255.0;
    let bg_blue: f32 = (background & 0x0000FF) as f32 / 255.0;

    let fg_red: f32 = ((colour & 0xFF0000) >> 16) as f32 / 255.0;
    let fg_green: f32 = ((colour & 0x00FF00) >> 8) as f32 / 255.0;
    let fg_blue: f32 = (colour & 0x0000FF) as f32 / 255.0;

    // Event loop.
    while let Some(e) = window.next() {
        // Key press for resetting grid.
        if let Some(button) = e.press_args() {
            use piston_window::Button::{Keyboard, Mouse};
            use piston_window::Key;

            match button {
                Keyboard(Key::Space) | Mouse(_) => {
                    if let Some(f) = file.clone() {
                        game_of_life.init_with_file(f).unwrap();
                    } else {
                        game_of_life.init_randomly(chance);
                    }
                }
                _ => (),
            }
        }

        // Drawing.
        window.draw_2d(&e, |c, g| {
            clear([bg_red, bg_green, bg_blue, 1.], g);

            for y in 0..game_of_life.board.len() {
                for x in 0..game_of_life.board[y].len() {
                    if game_of_life.board[y][x] {
                        rectangle(
                            [fg_red, fg_green, fg_blue, 1.],
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

            game_of_life.update();
        });
    }
}

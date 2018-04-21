extern crate clap;
extern crate game_of_life;
extern crate terminal_size;

use clap::{App, Arg};
use game_of_life::GameOfLife;
use terminal_size::*;

const ALIVE: char = '*';

fn main() {
    let Config {
        mut width,
        mut height,
        chance,
        sleep,
        file,
    } = Config::parse();

    if let Some((Width(w), Height(h))) = terminal_size() {
        width = w - 1;
        height = h - 1;
    }

    let mut game_of_life = GameOfLife::new(width as usize, height as usize);

    if let Some(f) = file {
        game_of_life.init_with_file(f).unwrap();
    } else {
        game_of_life.init_randomly(chance);
    }

    loop {
        // Clear screen.
        println!("\x1B[{}A", height + 1);

        game_of_life.update();

        let output = game_of_life.to_string_with_alive(ALIVE);
        print!("{}", output);

        if sleep > 0 {
            let sleep = std::time::Duration::from_millis(sleep);
            std::thread::sleep(sleep);
        }
    }
}

struct Config {
    pub width: u16,
    pub height: u16,
    pub chance: u8,
    pub sleep: u64,
    pub file: Option<String>,
}

impl Config {
    fn parse() -> Self {
        let matches = App::new("game-of-life")
        .version("0.3.0")
        .author("Splinter Suidman")
        .about("game-of-life emulates John Conway's game of life.\nPress ^C to quit.")
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .help("Change the width of the board (in cells).\nDefault: terminal width.")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .help("Change the height of the board (in cells).\nDefault: terminal height.")
            .takes_value(true))
        .arg(Arg::with_name("chance")
            .short("l")
            .long("chance")
            .help("Chance for randomly initialising board.\nExample: with '--chance 50' passed, cells will have a 50% chance of living.\nDefault: 15.")
            .takes_value(true))
        .arg(Arg::with_name("sleep")
            .long("sleep")
            .help("The amount of milliseconds the thread sleeps between every frame.\nDefault: 0.")
            .takes_value(true))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("The file that contains the board.\nIf this flag is passed, the board will be initialised with the board in the given file.\nDefault: None.")
            .takes_value(true))
        .get_matches();

        macro_rules! parse_or_default {
            ($name:expr, $default:expr) => {
                matches
                    .value_of($name)
                    .and_then(|s| match s.trim().parse() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    })
                    .unwrap_or($default)
            };
        }

        let width: u16 = parse_or_default!("width", 50);
        let height: u16 = parse_or_default!("height", 50);
        let chance: u8 = parse_or_default!("chance", 15);
        let sleep: u64 = parse_or_default!("sleep", 24);
        let file: Option<String> = matches.value_of("file").and_then(|s| Some(String::from(s)));

        Config {
            width,
            height,
            chance,
            sleep,
            file,
        }
    }
}

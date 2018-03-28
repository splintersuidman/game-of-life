extern crate clap;
extern crate piston_window;

mod lib;

use lib::GameOfLife;
use piston_window::*;
use clap::{App, Arg};

fn main() {
    let config = Config::parse();

    let mut game_of_life = GameOfLife::new(config.width as usize, config.height as usize);

    if let Some(f) = config.file.clone() {
        game_of_life.init_with_file(f).unwrap();
    } else {
        game_of_life.init_randomly(config.chance);
    }

    let name = if let Some(ref n) = game_of_life.name {
        format!("Game of Life - {}", n)
    } else {
        String::from("Game of Life")
    };

    // Create View for managing boards larger than a window
    let mut view = View::from_config(&config);

    // Create window.
    let mut window: PistonWindow =
        WindowSettings::new(name, [view.window_width, view.window_height])
            .exit_on_esc(true)
            .build()
            .unwrap();

    window.set_capture_cursor(view.capture_cursor);

    // Set event loop settings
    let mut settings = window.get_event_settings();
    settings.set_ups(0);
    settings.set_max_fps(config.fps);
    window.set_event_settings(settings);

    // Event loop.
    while let Some(e) = window.next() {
        // Key press for resetting grid.
        if let Some(button) = e.press_args() {
            use piston_window::Button::{Keyboard, Mouse};
            use piston_window::Key;

            match button {
                Keyboard(Key::Space) | Mouse(_) => {
                    if let Some(f) = config.file.clone() {
                        game_of_life.init_with_file(f).unwrap();
                    } else {
                        game_of_life.init_randomly(config.chance);
                    }
                }
                Keyboard(Key::C) => {
                    view.toggle_capture_cursor();
                    window.set_capture_cursor(view.capture_cursor);
                }
                _ => (),
            }
        }

        // On window resize
        e.resize(|width, height| {
            view.on_resize(width, height);
        });

        // On mouse movement
        e.mouse_relative(|x: f64, y: f64| {
            view.on_mouse_move(x, y);
        });

        // Drawing.
        window.draw_2d(&e, |c, g| {
            clear(config.background, g);

            for y in 0..view.cells_on_height {
                for x in 0..view.cells_on_width {
                    if game_of_life.board[y + view.y][x + view.x] {
                        rectangle(
                            config.foreground,
                            [
                                (x as f64) * view.cell_width,
                                (y as f64) * view.cell_width,
                                view.cell_width,
                                view.cell_width,
                            ],
                            c.transform,
                            g,
                        );
                    } else if config.view_border && (y + view.y == 0 || y + view.y + 1 == view.board_height || x + view.x == 0 || x + view.x + 1 == view.board_width) {
                        rectangle(
                            [0.5, 0.5, 0.5, 1.0],
                            [
                                (x as f64) * view.cell_width,
                                (y as f64) * view.cell_width,
                                view.cell_width,
                                view.cell_width,
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

struct View {
    pub cell_width: f64,
    pub base_cell_width: f64,

    pub y: usize,
    pub x: usize,

    pub precise_y: f64,
    pub precise_x: f64,

    pub capture_cursor: bool,

    pub cells_on_width: usize,
    pub cells_on_height: usize,

    pub board_width: usize,
    pub board_height: usize,

    pub window_width: u32,
    pub window_height: u32,
}

impl View {
    fn toggle_capture_cursor(&mut self) {
        self.capture_cursor = !self.capture_cursor;
    }

    fn from_config(config: &Config) -> Self {

        Self {
            y: 0,
            x: 0,

            precise_y: 0.0,
            precise_x: 0.0,

            cell_width: config.cell_width.into(),
            base_cell_width: config.cell_width.into(),

            capture_cursor: true,

            // limit initial window size to 600
            window_width: if config.width * config.cell_width > 600 {600} else {config.width * config.cell_width},
            window_height: if config.height * config.cell_width > 600 {600} else {config.height * config.cell_width},

            cells_on_width: (config.width * config.cell_width / config.cell_width) as usize,
            cells_on_height: (config.height * config.cell_width / config.cell_width) as usize,

            board_width: config.width as usize,
            board_height: config.height as usize,
        }
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;

        // reset to the base cell width
        self.cell_width = self.base_cell_width;

        self.cells_on_width = (self.window_width as f64 / self.cell_width) as usize;
        self.cells_on_height = (self.window_height as f64 / self.cell_width) as usize;

        if self.cells_on_width > self.board_width {
            self.cell_width = self.window_width as f64 / self.board_width as f64;
            self.cells_on_width = (self.window_width as f64 / self.cell_width) as usize;
            self.cells_on_height = (self.window_height as f64 / self.cell_width) as usize;
        }

        if self.cells_on_height > self.board_height {
            self.cell_width = self.window_height as f64 / self.board_height as f64;
            self.cells_on_height = (self.window_height as f64 / self.cell_width) as usize;
        }

        // trigger function to check for moving outside of the board
        self.on_mouse_move(0.0, 0.0);
    }

    fn on_mouse_move(&mut self, mouse_x: f64, mouse_y: f64) {
        if self.capture_cursor {
            // prevent y from moving outside of the board and update it
            if self.precise_y - mouse_y < 0.0 {
                self.precise_y = 0.0;
            } else if self.precise_y - mouse_y + self.cells_on_height as f64 > self.board_height as f64 {
                self.precise_y = (self.board_height - self.cells_on_height) as f64;
            } else {
                self.precise_y -= mouse_y;
            }

            // prevent x from moving outside of the board and update it
            if self.precise_x - mouse_x < 0.0 {
                self.precise_x = 0.0;
            } else if self.precise_x - mouse_x + self.cells_on_width as f64 > self.board_width as f64 {
                self.precise_x = (self.board_width - self.cells_on_width) as f64;
            } else {
                self.precise_x -= mouse_x;
            }

            self.y = self.precise_y as usize;
            self.x = self.precise_x as usize;
        }
    }
}

struct Config {
    pub width: u32,
    pub height: u32,
    pub cell_width: u32,
    pub chance: u8,
    pub fps: u64,
    pub file: Option<String>,
    pub foreground: [f32; 4],
    pub background: [f32; 4],
    pub view_border: bool,
}

impl Config {
    fn parse() -> Self {
        let matches = App::new("game-of-life")
        .version("0.3.0")
        .author("Splinter Suidman")
        .about("game-of-life emulates John Conway's game of life.\nPress Escape to exit, press C to toggle cursor capture and press Space or a mouse button to reinitialise grid.")
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .help("Change the width of the board (in cells).\nDefault: 50.")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .help("Change the height of the board (in cells).\nDefault: 50.")
            .takes_value(true))
        .arg(Arg::with_name("cell-width")
            .short("c")
            .long("cell-width")
            .help("Change width of a cell (in pixels).\nDefault: 10.")
            .takes_value(true))
        .arg(Arg::with_name("chance")
            .short("l")
            .long("chance")
            .help("Chance for randomly initialising board.\nExample: with '--chance 128' passed, cells will have a 50% chance of living.\nDefault: 220.")
            .takes_value(true))
        .arg(Arg::with_name("fps")
            .long("fps")
            .help("The amount of updates and frames that should be performed per second.\nThis is the maximum frames per second; that is, the actual fps could be less.\nDefault: 24.")
            .takes_value(true))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .help("The file that contains the board.\nIf this flag is passed, the board will be initialised with the board in the given file.\nDefault: None.")
            .takes_value(true))
        .arg(Arg::with_name("foreground")
            .long("foreground")
            .help("Change the foreground colour of the cells.\nThe colour should be passed as a hexidecimal RGB colour, example: FFFFFF for white, 000000 for black.\nDefault: 000000.")
            .takes_value(true))
        .arg(Arg::with_name("background")
            .long("background")
            .help("Change the background colour.\nThe colour should be passed as a hexidecimal RGB colour, example: FFFFFF for white, 000000 for black.\nDefault: FFFFFF.")
            .takes_value(true))
        .arg(Arg::with_name("view-border")
            .long("view-border")
            .help("Configures whether the border is visible on your screen.\nDefault: false.")
            .takes_value(true))
        .get_matches();

        macro_rules! parse_or_default {
            ($name: expr, $default: expr) => {
                matches
                    .value_of($name)
                    .and_then(|s| match s.trim().parse() {
                        Ok(v) => Some(v),
                        Err(_) => None,
                    })
                    .unwrap_or($default)
            };
        }

        let width: u32 = parse_or_default!("width", 50);
        let height: u32 = parse_or_default!("height", 50);
        let cell_width: u32 = parse_or_default!("cell-width", 10);
        let chance: u8 = parse_or_default!("chance", 220);
        let fps: u64 = parse_or_default!("fps", 24);
        let file: Option<String> = matches.value_of("file").and_then(|s| Some(String::from(s)));

        let foreground: u32 = matches
            .value_of("foreground")
            .and_then(|s| match u32::from_str_radix(&s, 16) {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .unwrap_or(0x000000);
        let background: u32 = matches
            .value_of("background")
            .and_then(|s| match u32::from_str_radix(&s, 16) {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .unwrap_or(0xFFFFFF);

        let background = [
            ((background & 0xFF0000) >> 16) as f32 / 255.0,
            ((background & 0x00FF00) >> 8) as f32 / 255.0,
            (background & 0x0000FF) as f32 / 255.0,
            1.,
        ];
        let foreground = [
            ((foreground & 0xFF0000) >> 16) as f32 / 255.0,
            ((foreground & 0x00FF00) >> 8) as f32 / 255.0,
            (foreground & 0x0000FF) as f32 / 255.0,
            1.,
        ];

        let view_border: bool = parse_or_default!("view-border", false);

        Config {
            width,
            height,
            cell_width,
            chance,
            fps,
            file,
            foreground,
            background,
            view_border,
        }
    }
}

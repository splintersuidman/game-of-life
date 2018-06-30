use super::clap::{App, Arg};

pub struct Config {
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
    pub fn parse() -> Self {
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
            .help("Chance for randomly initialising board.\nExample: with '--chance 50' passed, cells will have a 50% chance of living.\nDefault: 15.")
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
            .takes_value(false))
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

        let width: u32 = parse_or_default!("width", 50);
        let height: u32 = parse_or_default!("height", 50);
        let cell_width: u32 = parse_or_default!("cell-width", 10);
        let chance: u8 = parse_or_default!("chance", 15);
        let fps: u64 = parse_or_default!("fps", 24);
        let file: Option<String> = matches.value_of("file").and_then(|s| Some(String::from(s)));

        let foreground: u32 = matches
            .value_of("foreground")
            .and_then(|s| match u32::from_str_radix(&s, 16) {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .unwrap_or(0x00_00_00);
        let background: u32 = matches
            .value_of("background")
            .and_then(|s| match u32::from_str_radix(&s, 16) {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .unwrap_or(0xFF_FF_FF);

        let background = [
            ((background & 0xFF_00_00) >> 16) as f32 / 255.0,
            ((background & 0x00_FF_00) >> 8) as f32 / 255.0,
            (background & 0x00_00_FF) as f32 / 255.0,
            1.,
        ];
        let foreground = [
            ((foreground & 0xFF_00_00) >> 16) as f32 / 255.0,
            ((foreground & 0x00_FF_00) >> 8) as f32 / 255.0,
            (foreground & 0x00_00_FF) as f32 / 255.0,
            1.,
        ];

        let view_border: bool = matches.is_present("view-border");

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

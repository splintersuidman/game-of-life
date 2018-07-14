mod life105;
mod life106;
mod plaintext;
mod rle;

pub use self::life105::Life105;
pub use self::life106::Life106;
pub use self::plaintext::Plaintext;
pub use self::rle::RLE;

use super::CellState;
use std::fs::File;
use std::io::Read;
use std::fmt;

pub trait Parse {
    fn parse<S: AsRef<str>>(file: S) -> Result<Pattern, String>;
    fn is_type<S: AsRef<str>>(file: S) -> bool;
}

pub trait Serialise {
    // TODO: change output to generic write object.
    fn serialise<W: fmt::Write>(output: &mut W, pattern: Pattern) -> Result<(), fmt::Error>;
}

#[derive(Default)]
pub struct CellList {
    pub cells: Vec<(isize, isize)>,
    pub center: (isize, isize),
}

#[derive(Default)]
pub struct CellTable {
    pub cells: Vec<Vec<CellState>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Default)]
pub struct Pattern {
    pub cells: Vec<(isize, isize)>,
    pub metadata: Metadata,
}

impl Pattern {
    // TODO: change AsRef<str> to AsRef<Path> or something like that.
    pub fn parse_file<S: AsRef<str>>(filename: S) -> Result<Pattern, String> {
        let filename = filename.as_ref();

        // Read file and get rules from them.
        let mut file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => return Err(format!("Could not open file: {}", e)),
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(format!("Could not read file to string: {}", e));
        }

        if Life105::is_type(&contents) {
            Pattern::parse_file_with_type::<Life105>(&contents)
        } else if Life106::is_type(&contents) {
            Pattern::parse_file_with_type::<Life106>(&contents)
        } else if Plaintext::is_type(&contents) {
            Pattern::parse_file_with_type::<Plaintext>(&contents)
        } else {
            Pattern::parse_file_with_type::<RLE>(&contents)
        }
    }

    // TODO: change AsRef<str> to AsRef<Path> or something like that.
    pub fn parse_file_with_type<F: Parse>(file: &str) -> Result<Pattern, String> {
        F::parse(file)
    }

    pub fn serialise<F: Serialise>(self, output: &mut String) -> Result<(), fmt::Error> {
        F::serialise(output, self)
    }
}

#[derive(Default, Clone)]
pub struct Metadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub generation: Option<usize>,
}

// TODO: benchmark.
impl From<CellList> for CellTable {
    fn from(list: CellList) -> CellTable {
        use std::iter;

        let mut min_x = isize::max_value();
        let mut max_x = isize::min_value();
        let mut min_y = isize::max_value();
        let mut max_y = isize::min_value();

        for &(x, y) in list.cells.iter() {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_x {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }

        let mut table = CellTable::default();

        table.width = (max_x - min_x) as usize + 1;
        table.height = (max_y - min_y) as usize + 1;

        table.cells = iter::repeat(iter::repeat(CellState::Dead).take(table.width).collect())
            .take(table.height)
            .collect();

        for (x, y) in list.cells {
            table.cells[(y - min_y) as usize][(x - min_x) as usize] = CellState::Alive;
        }

        table
    }
}

// TODO: benchmark.
impl From<CellTable> for CellList {
    fn from(table: CellTable) -> CellList {
        let mut list = CellList::default();
        // TODO: appropriate to say center = (width / 2, height / 2)?
        list.center = (table.width as isize / 2, table.height as isize / 2);

        for y in 0..table.width {
            for x in 0..table.height {
                if table.cells[y][x] == CellState::Alive {
                    list.cells
                        .push((x as isize - list.center.0, y as isize - list.center.1));
                }
            }
        }

        list
    }
}

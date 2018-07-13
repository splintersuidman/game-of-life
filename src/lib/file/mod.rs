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

pub trait Parse {
    fn parse<S: AsRef<str>>(file: S) -> Result<Pattern, String>;
    fn is_type<S: AsRef<str>>(file: S) -> bool;
}

pub trait Serialise {
    // TODO: change output to generic write object.
    fn serialise(output: &mut String, pattern: Pattern) -> Result<(), String>;
}

#[derive(Default)]
pub struct CellList {
    cells: Vec<(isize, isize)>,
    center: (isize, isize),
}

#[derive(Default)]
pub struct CellTable {
    cells: Vec<Vec<CellState>>,
    width: usize,
    height: usize,
}

#[derive(Default)]
pub struct Pattern {
    pub cells: Vec<(isize, isize)>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
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

    pub fn serialise<F: Serialise>(self, output: &mut String) -> Result<(), String> {
        F::serialise(output, self)
    }
}

// TODO: benchmark + test.
impl From<CellList> for CellTable {
    fn from(list: CellList) -> CellTable {
        use std::iter;

        let mut outer_x = (isize::max_value(), isize::min_value());
        let mut outer_y = outer_x.clone();

        for (x, y) in list.cells.iter() {
            let (x, y) = (*x, *y);

            if x < outer_x.0 {
                outer_x.0 = x;
            }
            if x > outer_x.1 {
                outer_x.1 = x;
            }
            if y < outer_y.0 {
                outer_y.0 = y;
            }
            if y > outer_y.1 {
                outer_y.1 = y;
            }
        }

        let mut table = CellTable::default();

        table.width = (outer_x.1 - outer_x.0) as usize;
        table.height = (outer_y.1 - outer_y.0) as usize;

        table.cells = iter::repeat(iter::repeat(CellState::Dead).take(table.width).collect())
            .take(table.height)
            .collect();

        for (x, y) in list.cells {
            table.cells[(list.center.0 - x) as usize][(list.center.1 - y) as usize] =
                CellState::Alive;
        }

        table
    }
}

// TODO: benchmark + test.
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

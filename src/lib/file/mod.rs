mod life105;
mod life106;
mod plaintext;
mod rle;
mod cells;

pub use self::life105::Life105;
pub use self::life106::Life106;
pub use self::plaintext::Plaintext;
pub use self::rle::RLE;
pub use self::cells::{Cells, CellList, CellTable};

use super::{CellState, Rule};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub trait Parse {
    fn parse<S: AsRef<str>>(file: S) -> Result<Pattern, String>;
    fn is_type<S: AsRef<str>>(file: S) -> bool;
}

pub trait Serialise {
    fn serialise<W: fmt::Write>(output: &mut W, pattern: Pattern) -> Result<(), fmt::Error>;
}

#[derive(Clone, Default)]
pub struct Pattern {
    pub cells: Cells,
    pub metadata: Metadata,
}

impl Pattern {
    pub fn parse_file<P: AsRef<Path>>(filename: P) -> Result<Pattern, String> {
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
    pub rule: Rule,
}

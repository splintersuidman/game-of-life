use std::fs::File;
use std::io::Read;

pub mod plaintext;
pub mod life_106;
pub mod life_105;
pub mod rle;

/// Describes what type of file it is based on the file extension.
pub enum FileType {
    Life,
    PlainText,
    RLE,
}

impl FileType {
    /// Parses the file type from filename.
    pub fn from_filename<S: AsRef<str>>(s: &S) -> Option<FileType> {
        let s = s.as_ref();
        if s.ends_with("lif") || s.ends_with("life") {
            Some(FileType::Life)
        } else if s.ends_with("cells") {
            Some(FileType::PlainText)
        } else if s.ends_with("rle") {
            Some(FileType::RLE)
        } else {
            None
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn parse_file<S: AsRef<str>>(filename: S) -> Result<Vec<(isize, isize)>, String> {
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

        let file_type: FileType =
            FileType::from_filename(&filename).expect("Unrecognised file type.");

        match file_type {
            FileType::Life => {
                if life_106::is_life_106_file(&contents) {
                    life_106::parse_life_106_file(&contents)
                } else if life_105::is_life_105_file(&contents) {
                    life_105::parse_life_105_file(&contents)
                } else {
                    Err(String::from("File was classified as Life but it misses all of the known headers: `#Life 1.06` and `#Life 1.05`."))
                }
            }
            FileType::PlainText => {
                if plaintext::is_plaintext_file(&contents) {
                    plaintext::parse_plaintext_file(&contents)
                } else {
                    Err(String::from("File was classified as a plaintext file (`.cells`) but it doesn't start with `!Name: `."))
                }
            }
            FileType::RLE => rle::parse_rle_file(&contents),
        }
    }
}

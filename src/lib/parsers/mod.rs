mod plaintext;
mod life_106;
mod life_105;
mod run_length_encoded;

/// Describes what type of file it is based on the file extension.
pub enum FileType {
    Life,
    PlainText,
    RLE,
}

impl FileType {
    pub fn from_filename<S: ToString>(s: &S) -> Option<FileType> {
        let s = s.to_string();
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
    pub fn parse_file<S>(filename: S) -> Result<Vec<(isize, isize)>, String>
    where
        S: AsRef<str>,
    {
        use std::fs::File;
        use std::io::Read;

        // Read file and get rules from them.
        let mut file = match File::open(filename.as_ref()) {
            Ok(f) => f,
            Err(e) => return Err(format!("Could not open file: {}", e)),
        };

        let mut contents: &mut String = &mut String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(e) => return Err(format!("Could not read file to string: {}", e)),
        };

        let file_type: FileType =
            FileType::from_filename(&filename.as_ref()).expect("Unrecognised file type.");

        match file_type {
            FileType::Life => {
                if Parser::is_life_106_file(contents) {
                    life_106::parse_life_106_file(contents)
                } else if Parser::is_life_105_file(contents) {
                    life_105::parse_life_105_file(contents)
                } else {
                    Err(format!("File was classified as Life but it misses all of the known headers: `#Life 1.06` and `#Life 1.05`."))
                }
            }
            FileType::PlainText => {
                if Parser::is_plaintext_file(contents) {
                    plaintext::parse_plaintext_file(contents)
                } else {
                    Err(format!("File was classified as a plaintext file (`.cells`) but it doesn't start with `!Name: `."))
                }
            }
            FileType::RLE => run_length_encoded::parse_rle_file(contents),
        }
    }

    fn is_life_106_file<S: ToString>(s: &S) -> bool {
        s.to_string().starts_with("#Life 1.06")
    }

    fn is_life_105_file<S: ToString>(s: &S) -> bool {
        s.to_string().starts_with("#Life 1.05")
    }

    fn is_plaintext_file<S: ToString>(s: &S) -> bool {
        s.to_string().starts_with("!Name:")
    }
}

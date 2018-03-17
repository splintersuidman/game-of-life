mod plaintext;
mod life_106;
mod life_105;
mod run_length_encoded;

/// Describes what type of file it is based on the file extension.
pub enum FileType {
    Life,
    Plaintext,
    RLE,
    Unknown(String),
    None,
}

impl FileType {
    pub fn from_filename<S>(s: S) -> FileType
    where
        S: ToString,
    {
        let name: String = s.to_string();
        let extension: String = match name.rfind(".") {
            Some(x) => name[x + 1..].to_lowercase(), // exclude the `dot` as well
            None => String::from(""),
        };
        match extension.as_str() {
            "lif" | "life" => FileType::Life,
            "cells" => FileType::Plaintext,
            "rle" => FileType::RLE,
            "" => FileType::None,
            x => FileType::Unknown(x.to_string()),
        }
    }
}

pub struct Pattern {
    pub cells: Vec<(isize, isize)>,
    pub name: Option<String>,
}

impl Pattern {
    pub fn from_file<S>(filename: S) -> Result<Pattern, String>
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

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(e) => return Err(format!("Could not read file to string: {}", e)),
        };

        let file_type: FileType = FileType::from_filename(filename.as_ref());

        let pattern: Result<Pattern, String> = match file_type {
            FileType::Life => {
                if life_106::Parser::is_life_106_file(contents.clone()) {
                    life_106::Parser::parse_life_106_file(contents)
                } else if life_105::Parser::is_life_105_file(contents.clone()) {
                    life_105::Parser::parse_life_105_file(contents)
                } else {
                    Err(format!("File was classified as Life but it misses all of the known headers: `#Life 1.06` and `#Life 1.05`."))
                }
            }
            FileType::Plaintext => {
                if plaintext::Parser::is_plaintext_file(contents.clone()) {
                    plaintext::Parser::parse_plaintext_file(contents)
                } else {
                    Err(format!("File was classified as a plaintext file (`.cells`) but it doesn't start with `!Name: `."))
                }
            }
            FileType::RLE => run_length_encoded::Parser::parse_rle_file(contents),
            FileType::Unknown(s) => Err(format!("Unknown and/or unsupported file type: `{}`", s)),
            FileType::None => Err(format!("File doesn't appear to have a file extension.")),
        };

        let pattern: Pattern = pattern.unwrap();

        Ok(pattern)
    }

    pub fn empty() -> Pattern {
        Pattern {
            cells: Vec::new(),
            name: None,
        }
    }
}

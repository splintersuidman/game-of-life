mod plaintext;
mod life_106;

pub enum FileType {
    Life106,
    Plaintext,
    Unknown(String),
    None,
}

impl FileType {
    pub fn from_filename<S>(s: S) -> FileType
    where
        S: ToString
    {
        let name:String = s.to_string();
        let extension: &str = match name.rfind(".") {
            Some(x) => name.split_at(x+1).1, // exclude the `dot` as well
            None => "",
        };
        match extension {
            "lif" | "life" => FileType::Life106,
            "cells" => FileType::Plaintext,
            "" => FileType::None,
            x => FileType::Unknown(x.to_string()),
        }
    }
}

/// Checks whether a good classification was made and parses the file contents.
pub fn get_cell_rules<S>(file_type: FileType, contents: S) -> Result< Vec<(isize, isize)>, String >
where
    S: ToString
{
    let contents = contents.to_string();
    match file_type {
        FileType::Life106 => {
            if life_106::Parser::is_life_106_file(contents.clone()) {
                life_106::Parser::parse_life_106_file(contents)
            } else {
                Err(format!("File was classified as Life 1.06 but it misses it's header: `#Life 1.06`"))
            }
        },
        FileType::Plaintext => {
            if plaintext::Parser::is_plaintext_file(contents.clone()) {
                plaintext::Parser::parse_plaintext_file(contents)
            } else {
                Err(format!("File was classified as a plaintext file (`.cells`) but it doesn't start with `!Name: `."))
            }
        },
        FileType::Unknown(s) => {
            Err(format!("Unknown and/or unsupported file type: `{}`", s))
        },
        FileType::None => {
            Err(format!("File doesn't appear to have a file extension."))
        }
    }
}
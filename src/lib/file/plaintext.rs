use super::{Parse, Pattern, Serialise};
use std::fmt;

pub struct Plaintext;

impl Serialise for Plaintext {
    fn serialise<W: fmt::Write>(output: &mut W, pattern: Pattern) -> Result<(), fmt::Error> {
        // TODO: serialise.
        unimplemented!()
    }
}

impl Parse for Plaintext {
    fn is_type<S: AsRef<str>>(file: S) -> bool {
        file.as_ref().starts_with("!Name:")
    }

    fn parse<S: AsRef<str>>(file: S) -> Result<Pattern, String> {
        let file = file.as_ref();

        let mut pattern = Pattern::default();

        let mut metadata_lines = file.lines().take_while(|x| x.starts_with('!'));

        // Process name (!Name: name)
        if let Some(name) = metadata_lines.next() {
            // exlude the "!Name:" and remove surrounding whitespace
            let name: &str = name[6..].trim();
            pattern.metadata.name = Some(String::from(name));
        }

        // Process comments (lines starting with '!')
        for description in metadata_lines {
            // Check for other information
            if description.starts_with("!Author:") {
                let description = description[8..].trim();
                pattern.metadata.author = Some(String::from(description));
            } else {
                // Default, this line is a description
                let description = description[1..].trim();
                // and add to earlier description lines
                if let Some(d) = pattern.metadata.description {
                    pattern.metadata.description = Some(format!("{}\n{}", d, description));
                } else {
                    pattern.metadata.description = Some(String::from(description));
                }
            }
        }

        let lines = file.lines().skip_while(|x| x.starts_with('!'));

        for (y, line) in lines.enumerate() {
            for (x, token) in line.chars().enumerate() {
                match token {
                    // Cell is alive.
                    'O' => {
                        pattern.cells.push((x as isize, y as isize));
                    }
                    // Cell is dead.
                    '.' => {}
                    a => {
                        return Err(format!("Unexpected character `{}` while reading a plaintext file, expected `O` or `.`.", a));
                    }
                }
            }
        }

        Ok(pattern)
    }
}

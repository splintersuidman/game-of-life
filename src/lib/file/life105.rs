use super::{Parse, Pattern, Serialise};
use std::fmt;

pub struct Life105;

impl Serialise for Life105 {
    fn serialise<W: fmt::Write>(output: &mut W, pattern: Pattern) -> Result<(), fmt::Error> {
        // TODO: serialise.
        unimplemented!()
    }
}

impl Parse for Life105 {
    fn is_type<S: AsRef<str>>(file: S) -> bool {
        file.as_ref().starts_with("#Life 1.05")
    }

    fn parse<S: AsRef<str>>(file: S) -> Result<Pattern, String> {
        let file = file.as_ref();

        let mut pattern = Pattern::default();

        let metadata_lines: String = file
            .lines()
            .filter(|x| x.starts_with("#D"))
            .map(|x| format!("{}\n", x[2..].trim()))
            .collect();

        if metadata_lines.is_empty() {
            pattern.metadata.description = None;
        } else {
            pattern.metadata.description = Some(metadata_lines);
        }

        // TODO: support #R (rule).

        // Remove all lines beginning with "#", except the ones with "#P" because they give information
        // about the blocks.
        let lines = file
            .lines()
            .filter(|x| !x.starts_with('#') || x.starts_with("#P"));

        let mut y: isize = -1;
        let mut base_x: isize = 0;
        for line in lines {
            y += 1;
            let mut x = base_x;
            if line.starts_with("#P") {
                let mut data = line.split_whitespace().skip(1);

                base_x = match data.next() {
                    Some(x) => match x.parse() {
                        Ok(x) => x,
                        Err(e) => return Err(format!("Could not read data for x: {}", e)),
                    }
                    None => return Err(String::from("Could not find data for x in line starting with `#P` while reading a Life 1.05 file.")),
                };

                y = match data.next() {
                    Some(y) => match y.parse() {
                        Ok(y) => y,
                        Err(e) => return Err(format!("Could not read data for y: {}.", e))
                    },
                    None => return Err(String::from("Could not find data for x in line starting with `#P` while reading a Life 1.05 file.")),
                };
            } else {
                for token in line.chars() {
                    match token {
                        // Cell is dead.
                        '.' => {}
                        // Cell is alive.
                        '*' => {
                            pattern.cells.push((x, y));
                        }
                        c => {
                            return Err(format!("Unexpected character `{}` while reading a Life 1.05 file, expected `.` or `*`.", c));
                        }
                    }
                    x += 1;
                }
            }
        }

        Ok(pattern)
    }
}

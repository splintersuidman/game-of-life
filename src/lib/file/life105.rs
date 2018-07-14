use super::{Parse, Pattern, Rule, Serialise};
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

        // Skip header.
        let mut lines = file.lines().skip(1).peekable();

        let mut description = String::new();
        while lines.peek().map(|l| l.starts_with("#D")) == Some(true) {
            // We can safely unwrap.
            for ch in lines.next().unwrap().chars().skip(3) {
                description.push(ch);
            }
        }

        pattern.metadata.description = if description.is_empty() {
            None
        } else {
            Some(description)
        };

        if lines.peek().map(|l| l.starts_with("#N")) == Some(true) {
            lines.next();
            pattern.metadata.rule = Rule::normal();
        } else if lines.peek().map(|l| l.starts_with("#R")) == Some(true) {
            let mut line = lines.next().unwrap().chars().skip(3).peekable();
            pattern.metadata.rule = Rule::empty();

            while line.peek().map(|ch| ch != &'/') == Some(true) {
                let ch = line.next().unwrap();
                pattern
                    .metadata
                    .rule
                    .set_survival((ch as u8 - b'0') as usize, true);
            }

            // Read '/'.
            line.next();

            for ch in line {
                pattern
                    .metadata
                    .rule
                    .set_birth((ch as u8 - b'0') as usize, true);
            }
        }

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

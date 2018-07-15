use super::{CellList, Cells, CellTable, CellState, Parse, Pattern, Rule, Serialise};
use std::fmt;

pub struct RLE;

impl Serialise for RLE {
    fn serialise<W: fmt::Write>(output: &mut W, pattern: Pattern) -> Result<(), fmt::Error> {
        if let Some(name) = pattern.metadata.name {
            writeln!(output, "#N {}", name)?;
        }

        if let Some(author) = pattern.metadata.author {
            writeln!(output, "#O {}", author)?;
        }

        if let Some(description) = pattern.metadata.description {
            for line in description.lines() {
                writeln!(output, "#C {}", line)?;
            }
        }

        let cells: CellTable = pattern.cells.into();

        write!(output, "x = {}, y = {}, rule = B", cells.width, cells.height)?;
        for i in 0..=8 {
            if pattern.metadata.rule.birth(i) {
                write!(output, "{}", i)?;
            }
        }
        write!(output, "/S")?;
        for i in 0..=8 {
            if pattern.metadata.rule.survival(i) {
                write!(output, "{}", i)?;
            }
        }
        writeln!(output, "")?;

        let mut data: Vec<(usize, char)> = Vec::new();

        for row in cells.into_iter() {
            for cell in row {
                let ch = if cell == CellState::Alive {
                    'o'
                } else {
                    'b'
                };

                if data.last().map(|v| v.1) == Some(ch) {
                    data.last_mut().unwrap().0 += 1;
                } else {
                    data.push((1, ch));
                }
            }

            let last = data.last().map(|v| v.1);
            if last == Some('b') {
                data.pop();
            }
            if last == Some('$') {
                data.last_mut().unwrap().0 += 1;
            } else {
                data.push((1, '$'));
            }
        }

        // The last newline(s) can be omitted.
        data.pop();

        // TODO: line limit?

        for cells in data {
            if cells.0 > 1 {
                write!(output, "{}", cells.0)?;
            }
            write!(output, "{}", cells.1)?;
        }

        write!(output, "!")
    }
}

impl Parse for RLE {
    /// NOTE: we can't be sure a file is an RLE file, so this always returns `true`.
    /// RLE should therefore be tried to parse after the other file types.
    fn is_type<S: AsRef<str>>(_: S) -> bool {
        false
    }

    fn parse<S: AsRef<str>>(file: S) -> Result<Pattern, String> {
        let file = file.as_ref();
        let mut pattern = Pattern::default();
        let mut cells = CellList::default();

        let mut lines = file.lines().peekable();

        // Parse all metadata lines, beginning with a '#'.
        while lines.peek().map(|l| l.starts_with('#')) == Some(true) {
            let mut linedata = lines.next().unwrap().chars().skip(1);
            match linedata.next() {
                Some('N') => {
                    // Name
                    let name: String = linedata.collect();
                    let name = name.trim();
                    if !name.is_empty() {
                        pattern.metadata.name = Some(String::from(name));
                    }
                }
                Some('C') | Some('c') => {
                    // Comment or description
                    let description: String = linedata.collect();
                    let description = description.trim();
                    if let Some(d) = pattern.metadata.description {
                        pattern.metadata.description = Some(format!("{}\n{}", d, description));
                    } else {
                        pattern.metadata.description = Some(String::from(description));
                    }
                }
                Some('O') => {
                    // Author
                    let author: String = linedata.collect();
                    let author = author.trim();
                    pattern.metadata.author = Some(String::from(author));
                }
                Some('r') => {
                    let rule: String = linedata.collect();
                    let rule = rule.trim();
                    let mut line = rule.chars().peekable();
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
                Some(unknown_char) => {
                    return Err(format!(
                        "Unknown combination #{} in metadata of .rle file.",
                        unknown_char
                    ));
                }
                None => {}
            }
        }

        // x = m, y = n
        let _header = match lines.next() {
            Some(v) => v,
            None => return Err(String::from("The header for this `.rle` file could not be found because there were no (uncommented) lines.")),
        };

        // TODO: process header information

        let data: String = lines.collect();
        let data = data.split('$');

        let mut y: isize = 0;
        for line in data {
            let mut amount: isize = 0;
            let mut x = 0;
            for c in line.chars() {
                match c {
                    'b' | '.' => {
                        // Off state
                        if amount == 0 {
                            // Not preceded by a number
                            x += 1;
                        } else {
                            x += amount;
                            amount = 0;
                        }
                    }
                    'o' | 'A' => {
                        // On state
                        if amount == 0 {
                            // Not preceded by a number
                            cells.push((x, y));
                            x += 1;
                        } else {
                            for i in 0..amount {
                                cells.push((x + i, y));
                            }
                            x += amount;
                            amount = 0;
                        }
                    }
                    ch @ '0'...'9' => amount = amount * 10 + (ch as u8 - b'0') as isize,
                    '!' => {
                        // The end of this pattern was reached
                        pattern.cells = Cells::List(cells);
                        return Ok(pattern);
                    }
                    unknown => {
                        return Err(format!(
                            "Unexpected character `{}` while reading data from a `.rle` file.",
                            unknown
                        ))
                    }
                }
            }
            if amount != 0 {
                y += amount;
            } else {
                y += 1;
            }
        }

        pattern.cells = Cells::List(cells);

        Ok(pattern)
    }
}

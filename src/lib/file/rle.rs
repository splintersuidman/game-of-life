use super::{Parse, Pattern, Serialise};

pub struct RLE;

impl Serialise for RLE {
    fn serialise(output: &mut String, pattern: Pattern) -> Result<(), String> {
        // TODO: serialise.
        unimplemented!()
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

        // Metadata
        let metadata = file.lines().take_while(|x| x.starts_with('#'));

        for line in metadata {
            let mut linedata = line.chars().skip(1);
            match linedata.next() {
                Some('N') => {
                    // Name
                    let name: String = linedata.collect();
                    let name = name.trim();
                    if !name.is_empty() {
                        pattern.name = Some(String::from(name));
                    }
                }
                Some('C') | Some('c') => {
                    // Comment or description
                    let description: String = linedata.collect();
                    let description = description.trim();
                    if let Some(d) = pattern.description {
                        pattern.description = Some(format!("{}\n{}", d, description));
                    } else {
                        pattern.description = Some(String::from(description));
                    }
                }
                Some('O') => {
                    // Author
                    let author: String = linedata.collect();
                    let author = author.trim();
                    pattern.author = Some(String::from(author));
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

        // Remove all of the lines starting with `#`
        let mut lines = file.lines().skip_while(|x| x.starts_with('#'));

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
                            pattern.cells.push((x, y));
                            x += 1;
                        } else {
                            for i in 0..amount {
                                pattern.cells.push((x + i, y));
                            }
                            x += amount;
                            amount = 0;
                        }
                    }
                    // TODO(splintah): rewrite because this is stupid (no offence).
                    '0' => amount *= 10,
                    '1' => amount = amount * 10 + 1,
                    '2' => amount = amount * 10 + 2,
                    '3' => amount = amount * 10 + 3,
                    '4' => amount = amount * 10 + 4,
                    '5' => amount = amount * 10 + 5,
                    '6' => amount = amount * 10 + 6,
                    '7' => amount = amount * 10 + 7,
                    '8' => amount = amount * 10 + 8,
                    '9' => amount = amount * 10 + 9,
                    '!' => {
                        // The end of this pattern was reached
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

        Ok(pattern)
    }
}

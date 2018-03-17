pub struct Parser;

impl Parser {
    /// Check whether the given file (read to string) is a Life 1.06 file.
    /// A file is a Life 1.06 file when the first line equals: `#Life 1.06`.
    pub fn is_life_106_file<S>(s: S) -> bool
    where
        S: ToString,
    {
        let s = s.to_string();
        let mut lines = s.lines();

        if let Some(s) = lines.next() {
            s == "#Life 1.06"
        } else {
            false
        }
    }

    /// Parse a Life 1.06 file to a `Pattern`.
    pub fn parse_life_106_file<S>(s: S) -> Result<super::Pattern, String>
    where
        S: ToString,
    {
        let s = s.to_string();
        // Skip first line, because it is the header.
        let lines = s.lines().skip(1);

        let mut pattern = super::Pattern::empty();

        for line in lines {
            // Skip empty lines.
            if line.is_empty() {
                continue;
            }

            let mut line_split = line.split_whitespace();
            let x = match line_split.next() {
                None => return Err(format!("could not find x in `{}`", line)),
                Some(v) => v,
            };
            let y = match line_split.next() {
                None => return Err(format!("could not find y in `{}`", line)),
                Some(v) => v,
            };

            let x: isize = match x.trim().parse() {
                Err(_) => return Err(format!("could not parse x as number: `{}`", x)),
                Ok(v) => v,
            };
            let y: isize = match y.trim().parse() {
                Err(_) => return Err(format!("could not parse x as number: `{}`", y)),
                Ok(v) => v,
            };

            pattern.cells.push((x, y));
        }

        Ok(pattern)
    }
}

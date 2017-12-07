pub struct Parser;

impl Parser {
    /// Check whether the given file (read to string) is a Life 1.06 file.
    /// A file is a Life 1.06 file when the first line equals: `#Life 1.06`.
    pub fn is_life_106_file<S>(s: S) -> bool
    where
        S: ToString
    {
        let s = s.to_string();
        let mut lines = s.lines();

        if let Some(s) = lines.next() {
            if s == "#Life 1.06" {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Parse a Life 1.06 file to a `Vec<(isize, isize)>`.
    pub fn parse_life_106_file<S>(s: S) -> Result<Vec<(isize, isize)>, String>
    where
        S: ToString
    {
        let s = s.to_string();
        // Skip first line, because it is the header.
        let lines = s.lines().skip(1);

        let mut cells: Vec<(isize, isize)> = Vec::new();

        for line in lines {
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

            cells.push((x, y));
        }

        Ok(cells)
    }
}

pub fn is_life_106_file<S: AsRef<str>>(s: &S) -> bool {
    s.as_ref().starts_with("#Life 1.06")
}

pub fn parse_life_106_file<S: AsRef<str>>(s: &S) -> Result<Vec<(isize, isize)>, String> {
    let s = s.as_ref();

    // Skip first line, because it is the header.
    let lines = s.lines().skip(1);

    let mut cells = Vec::new();

    for line in lines.filter(|s| !s.is_empty()) {
        let mut line_split = line.split_whitespace();

        let x = match line_split.next() {
            None => return Err(format!("could not find x in `{}`", line)),
            Some(v) => v,
        };
        let y = match line_split.next() {
            None => return Err(format!("could not find y in `{}`", line)),
            Some(v) => v,
        };

        let x = match x.trim().parse() {
            Err(_) => return Err(format!("could not parse x as number: `{}`", x)),
            Ok(v) => v,
        };
        let y = match y.trim().parse() {
            Err(_) => return Err(format!("could not parse x as number: `{}`", y)),
            Ok(v) => v,
        };

        cells.push((x, y));
    }

    Ok(cells)
}
